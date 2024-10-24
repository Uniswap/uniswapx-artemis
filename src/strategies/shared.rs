use crate::collectors::uniswapx_route_collector::RoutedOrder;
use anyhow::Result;
use async_trait::async_trait;
use bindings_uniswapx::{
    erc20::ERC20, shared_types::SignedOrder, swap_router_02_executor::SwapRouter02Executor,
};
use ethers::{
    abi::{ethabi, ParamType, Token},
    providers::Middleware,
    types::{transaction::eip2718::TypedTransaction, Address, Bytes, H160, U256},
};
use std::sync::Arc;
use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug)]
pub enum ReactorAddress {
    DutchV2,
    DutchV3,
    Priority,
}

impl ReactorAddress {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReactorAddress::DutchV2 => DUTCHV2_REACTOR_ADDRESS,
            ReactorAddress::DutchV3 => DUTCHV3_REACTOR_ADDRESS,
            ReactorAddress::Priority => PRIORITY_REACTOR_ADDRESS,
        }
    }
}

const DUTCHV2_REACTOR_ADDRESS: &str = "0x00000011F84B9aa48e5f8aA8B9897600006289Be";
const DUTCHV3_REACTOR_ADDRESS: &str = "0xB274d5F4b833b61B340b654d600A864fB604a87c";
const PRIORITY_REACTOR_ADDRESS: &str = "0x000000001Ec5656dcdB24D90DFa42742738De729";
const SWAPROUTER_02_ADDRESS: &str = "0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45";
pub const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

#[async_trait]
pub trait UniswapXStrategy<M: Middleware + 'static> {
    // builds a transaction to fill an order
    async fn build_fill(
        &self,
        client: Arc<M>,
        executor_address: &str,
        signed_orders: Vec<SignedOrder>,
        reactor_address: ReactorAddress,
        RoutedOrder { request, route }: &RoutedOrder,
    ) -> Result<TypedTransaction> {
        let chain_id: U256 = client.get_chainid().await?;
        let fill_contract =
            SwapRouter02Executor::new(H160::from_str(executor_address)?, client.clone());

        let token_in: H160 = H160::from_str(&request.token_in)?;
        let token_out: H160 = H160::from_str(&request.token_out)?;

        let swaprouter_02_approval = self
            .get_tokens_to_approve(
                client.clone(),
                token_in,
                executor_address,
                SWAPROUTER_02_ADDRESS,
            )
            .await?;

        let reactor_approval = self
            .get_tokens_to_approve(client.clone(), token_out, executor_address, reactor_address.as_str())
            .await?;

        // Strip off function selector
        let multicall_bytes = &route.method_parameters.calldata[10..];

        // Decode multicall into [Uint256, bytes[]] (deadline, multicallData)
        let decoded_multicall_bytes = ethabi::decode(
            &[
                ParamType::Uint(256),
                ParamType::Array(Box::new(ParamType::Bytes)),
            ],
            &Bytes::from_str(multicall_bytes).expect("Failed to decode multicall bytes"),
        );

        let decoded_multicall_bytes = match decoded_multicall_bytes {
            Ok(data) => data[1].clone(), // already in bytes[]
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to decode multicall bytes: {}", e));
            }
        };

        // abi encode as [tokens to approve to swap router 02, tokens to approve to reactor,  multicall data]
        //               [address[], address[], bytes[]]
        let calldata = ethabi::encode(&[
            Token::Array(swaprouter_02_approval),
            Token::Array(reactor_approval),
            decoded_multicall_bytes,
        ]);
        let mut call = fill_contract.execute_batch(signed_orders, Bytes::from(calldata));
        Ok(call.tx.set_chain_id(chain_id.as_u64()).clone())
    }

    fn current_timestamp(&self) -> Result<u64> {
        let start = SystemTime::now();
        Ok(start.duration_since(UNIX_EPOCH)?.as_secs())
    }

    async fn get_tokens_to_approve(
        &self,
        client: Arc<M>,
        token: Address,
        from: &str,
        to: &str,
    ) -> Result<Vec<Token>, anyhow::Error> {
        if token == Address::zero() {
            return Ok(vec![]);
        }
        let token_contract = ERC20::new(token, client.clone());
        let allowance = token_contract
            .allowance(
                H160::from_str(from).expect("Error encoding from address"),
                H160::from_str(to).expect("Error encoding from address"),
            )
            .await
            .expect("Failed to get allowance");
        if allowance < U256::MAX / 2 {
            Ok(vec![Token::Address(token)])
        } else {
            Ok(vec![])
        }
    }
}
