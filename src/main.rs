use alloy::rpc::client::ClientBuilder;
use anyhow::Result;
use backoff::ExponentialBackoff;
use clap::{ArgGroup, Parser};

use artemis_core::engine::Engine;
use artemis_core::types::{CollectorMap, ExecutorMap};
use collectors::uniswapx_order_collector::OrderType;
use collectors::{
    block_collector::BlockCollector, uniswapx_order_collector::UniswapXOrderCollector,
    uniswapx_route_collector::UniswapXRouteCollector,
};
use alloy::{
    hex,
    network::AnyNetwork,
    providers::{DynProvider, ProviderBuilder, WsConnect},
    pubsub::{ConnectionHandle, PubSubConnect},
    signers::local::PrivateKeySigner,
    transports::{impl_future, TransportResult},
};
use executors::dutch_executor::DutchExecutor;
use executors::queued_executor::QueuedExecutor;
use std::collections::HashMap;
use std::sync::Arc;
use strategies::dutchv3_strategy::UniswapXDutchV3Fill;
use strategies::keystore::KeyStore;
use strategies::priority_strategy::UniswapXPriorityFill;
use strategies::{
    types::{Action, Config, Event},
    uniswapx_strategy::UniswapXUniswapFill,
};
use tokio::sync::mpsc::channel;
use tracing::{error, info, warn, Level};
use tracing_subscriber::{filter, prelude::*};

pub mod aws_utils;
pub mod collectors;
pub mod executors;
pub mod shared;
pub mod strategies;

/// CLI Options.
#[derive(Parser, Debug)]
#[command(group(
    ArgGroup::new("key_source")
        .args(&["private_key", "private_key_file", "aws_secret_arn"])
        .required(true)
))]
#[command(group(
    ArgGroup::new("aws_features")
        .args(&["cloudwatch_metrics"])
))]
pub struct Args {
    /// Ethereum node WSS endpoint.
    #[arg(long, required = false)]
    pub wss: Option<String>,

    /// Ethereum node HTTP endpoint.
    #[arg(long, required = false)]
    pub http: Option<String>,

    /// Private key for sending txs.
    #[arg(long, group = "key_source")]
    pub private_key: Option<String>,

    /// Path to file containing mapping between public address and private key.
    #[arg(long, group = "key_source")]
    pub private_key_file: Option<String>,

    /// AWS secret arn for fetching private key.
    /// This is a secret manager arn that contains the private key as plain text.
    #[arg(long, group = "key_source")]
    pub aws_secret_arn: Option<String>,

    /// Percentage of profit to pay in gas.
    #[arg(long, required = false)]
    pub bid_percentage: Option<u128>,

    /// Determines how aggressive to scale the fallback bids
    /// 100 (default) = 1% of the profit
    #[arg(long, required = false)]
    pub fallback_bid_scale_factor: Option<u64>,

    /// Minimum block percentage buffer for priority orders.
    /// This determines how much time to wait before the target block to submit the fill transaction.
    /// Example: 120 = 120% of the block time which would be 2.4 seconds with a block time of 2 seconds.
    #[arg(long, required = false)]
    pub min_block_percentage_buffer: Option<u64>,

    /// Private key for sending txs.
    #[arg(long, required = true)]
    pub executor_address: String,

    /// Whether to use revert protection for priority orders.
    #[arg(long, required = false)]
    pub revert_protection: bool,

    /// Order type to use.
    #[arg(long, required = true)]
    pub order_type: OrderType,

    /// Enable CloudWatch logging
    #[arg(long, group = "aws_features")]
    pub cloudwatch_metrics: bool,

    /// chain id
    #[arg(long, required = true)]
    pub chain_id: u64,

    /// Optional UniswapX API Key
    #[arg(long)]
    pub uniswapx_api_key: Option<String>,
}

/// Retrying websocket connection using exponential backoff
/// https://github.com/alloy-rs/alloy/issues/690#issuecomment-2173956489
#[derive(Clone, Debug)]
pub struct RetryWsConnect(WsConnect);

impl PubSubConnect for RetryWsConnect {
    fn is_local(&self) -> bool {
        self.0.is_local()
    }

    fn connect(&self) -> impl_future!(<Output = TransportResult<ConnectionHandle>>) {
        self.0.connect()
    }

    async fn try_reconnect(&self) -> TransportResult<ConnectionHandle> {
        warn!("Trying to reconnect to ws");

        backoff::future::retry(ExponentialBackoff::default(), || async {
            Ok(self.0.try_reconnect().await?)
        })
        .await
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up tracing and parse args.
    let filter = filter::Targets::new()
        .with_target("artemis_core", Level::INFO)
        .with_target("uniswapx_artemis", Level::INFO);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                // this needs to be set to false, otherwise ANSI color codes will
                // show up in a confusing manner in CloudWatch logs.
                .with_ansi(false)
                // remove the name of the function from every log entry
                .with_target(false),
        )
        .with(filter)
        .init();

    let args = Args::parse();

    // Set up ethers provider.
    let chain_id = args.chain_id;
    let mut client = None;
    let mut sender_client = None;
    
    if let Some(wss) = args.wss {
        let ws = WsConnect::new(wss.as_str());
        let retry_ws = RetryWsConnect(ws);
        let wss_client = ClientBuilder::default().pubsub(retry_ws).await?;

        let wss_provider = Arc::new(DynProvider::<AnyNetwork>::new(
            ProviderBuilder::new()
                .network::<AnyNetwork>()
                .on_client(wss_client)
        ));
        client = Some(wss_provider.clone());
        sender_client = Some(wss_provider.clone());
        info!("Using WSS provider: {}", wss);
    }

    // Initialize HTTP provider if specified
    if let Some(http_endpoint) = args.http {
        info!("Using HTTP provider: {}", http_endpoint);
        let http_client = ClientBuilder::default().http(http_endpoint.parse()?);
        let http_provider = Arc::new(DynProvider::<AnyNetwork>::new(
            ProviderBuilder::new()
                .network::<AnyNetwork>()
                .on_client(http_client)
        ));
        // prefer http provider for sending txs
        sender_client = Some(http_provider.clone());
        // prefer wss provider for fetching blocks
        if !client.is_some() {
            client = Some(http_provider.clone());
        }
    }
    if !client.is_some() {
        panic!("No provider found. Please provide either a WSS endpoint (--wss) or an HTTP endpoint (--http).");
    }

    let mut key_store = Arc::new(KeyStore::new());

    if let Some(aws_secret_arn) = args.aws_secret_arn {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_secretsmanager::Client::new(&config);
        let pk_mapping_json = client
            .get_secret_value()
            .secret_id(aws_secret_arn)
            .send()
            .await
            .expect("could not get private key secret")
            .secret_string
            .expect("secret string not found");
        let pk_mapping = serde_json::from_str::<HashMap<String, String>>(&pk_mapping_json)
            .expect("could not parse private key mapping");
        // load into keystore
        for (address, pk) in pk_mapping {
            Arc::make_mut(&mut key_store).add_key(address, pk).await;
        }
    } else if let Some(pk_file) = args.private_key_file {
        let pk_mapping_json = std::fs::read_to_string(pk_file).expect("could not read pk file");
        let pk_mapping = serde_json::from_str::<HashMap<String, String>>(&pk_mapping_json)
            .expect("could not parse private key mapping");
        // load into keystore
        for (address, pk) in pk_mapping {
            Arc::make_mut(&mut key_store).add_key(address, pk).await;
        }
    } else {
        let pk = args.private_key.clone().unwrap();
        let wallet: PrivateKeySigner = pk.parse().expect("can not parse private key");
        let address = wallet.address();
        Arc::make_mut(&mut key_store)
            .add_key(hex::encode(address), pk)
            .await;
    }
    info!("Key store initialized with {} keys", key_store.len());

    // Set up engine.
    let mut engine = Engine::default();

    // Set up block collector.
    let block_collector = Box::new(BlockCollector::new(client.clone().unwrap()));
    let block_collector = CollectorMap::new(block_collector, Event::NewBlock);
    engine.add_collector(Box::new(block_collector));

    let (batch_sender, batch_receiver) = channel(512);
    let (route_sender, route_receiver) = channel(512);

    let uniswapx_order_collector = Box::new(UniswapXOrderCollector::new(
        chain_id,
        args.order_type.clone(),
        args.executor_address.clone(),
        args.uniswapx_api_key,
    ));
    let uniswapx_order_collector = CollectorMap::new(uniswapx_order_collector, |e| {
        Event::UniswapXOrder(Box::new(e))
    });
    engine.add_collector(Box::new(uniswapx_order_collector));

    let cloudwatch_client = if args.cloudwatch_metrics {
        let config = aws_config::load_from_env().await;
        Some(Arc::new(aws_sdk_cloudwatch::Client::new(&config)))
    } else {
        None
    };

    let uniswapx_route_collector = Box::new(UniswapXRouteCollector::new(
        chain_id,
        batch_receiver,
        route_sender,
        args.executor_address.clone(),
        cloudwatch_client.clone(),
    ));
    let uniswapx_route_collector = CollectorMap::new(uniswapx_route_collector, |e| {
        Event::UniswapXRoute(Box::new(e))
    });
    engine.add_collector(Box::new(uniswapx_route_collector));

    let config = Config {
        bid_percentage: args.bid_percentage,
        fallback_bid_scale_factor: args.fallback_bid_scale_factor,
        min_block_percentage_buffer: args.min_block_percentage_buffer,
        executor_address: args.executor_address,
        revert_protection: args.revert_protection,
    };

    match &args.order_type {
        OrderType::DutchV2 => {
            let uniswapx_strategy = UniswapXUniswapFill::new(
                client.clone().unwrap(),
                config.clone(),
                batch_sender,
                route_receiver,
                cloudwatch_client.clone(),
                chain_id,
            );
            engine.add_strategy(Box::new(uniswapx_strategy));
        }
        OrderType::DutchV3 => {
            let uniswapx_strategy = UniswapXDutchV3Fill::new(
                client.clone().unwrap(),
                cloudwatch_client.clone(),
                config.clone(),
                batch_sender,
                route_receiver,
                key_store.get_address().unwrap(),
                chain_id,
            );
            engine.add_strategy(Box::new(uniswapx_strategy));
        }
        OrderType::Priority => {
            let priority_strategy = UniswapXPriorityFill::new(
                client.clone().unwrap(),
                cloudwatch_client.clone(),
                config.clone(),
                batch_sender,
                route_receiver,
                chain_id,
            );

            engine.add_strategy(Box::new(priority_strategy));
        }
    }

    let queued_executor = Box::new(QueuedExecutor::new(
        client.clone().unwrap(),
        sender_client.clone().unwrap(),
        key_store.clone(),
        cloudwatch_client.clone(),
    ));

    let queued_executor = ExecutorMap::new(queued_executor, |action| match action {
        Action::SubmitPublicTx(tx) => Some(tx),
        _ => None,
    });

    let protect_executor = Box::new(DutchExecutor::new(
        client.clone().unwrap(),
        sender_client.clone().unwrap(),
        key_store.clone(),
        cloudwatch_client.clone(),
    ));

    let protect_executor = ExecutorMap::new(protect_executor, |action| match action {
        Action::SubmitTx(tx) => Some(tx),
        // No op for public transactions
        _ => None,
    });


    engine.add_executor(Box::new(queued_executor));
    engine.add_executor(Box::new(protect_executor));

    // Start engine.
    match engine.run().await {
        Ok(mut set) => {
            while let Some(res) = set.join_next().await {
                match res {
                    Ok(res) => {
                        info!("res: {:?}", res);
                    }
                    Err(e) => {
                        info!("error: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            error!("Engine run error: {:?}", e);
        }
    }
    Ok(())
}
