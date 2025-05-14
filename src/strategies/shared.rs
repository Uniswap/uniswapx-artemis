use crate::collectors::uniswapx_route_collector::RoutedOrder;
use alloy::{
    hex,
    network::{AnyNetwork, TransactionBuilder},
    primitives::{Address, Bytes, U256},
    providers::{DynProvider, Provider},
    rpc::types::TransactionRequest,
    serde::WithOtherFields,
    sol,
};
use anyhow::Result;
use async_trait::async_trait;
use bindings_uniswapx::{
    basereactor::BaseReactor::SignedOrder, erc20::ERC20,
    universalrouterexecutor::UniversalRouterExecutor,
};
use ethabi::{ethereum_types::H160, Token};
use std::{
    str::FromStr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

const REACTOR_ADDRESS: &str = "0x00000011F84B9aa48e5f8aA8B9897600006289Be";
const PERMIT2_ADDRESS: &str = "0x000000000022D473030F116dDEE9F6B43aC78BA3";
pub const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
const ARBITRUM_GAS_PRECOMPILE: &str = "0x000000000000000000000000000000000000006C";

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract GasPrecompileContract {
        function getMinimumGasPrice() external view returns (uint256);
    }
}

#[async_trait]
pub trait UniswapXStrategy {
    // builds a transaction to fill an order
    async fn build_fill(
        &self,
        client: Arc<DynProvider<AnyNetwork>>,
        executor_address: &str,
        signed_orders: Vec<SignedOrder>,
        RoutedOrder { request, route, .. }: &RoutedOrder,
    ) -> Result<WithOtherFields<TransactionRequest>> {
        let chain_id = client.get_chain_id().await?;
        let fill_contract =
            UniversalRouterExecutor::new(Address::from_str(executor_address)?, client.clone());

        let token_in = Address::from_str(&request.token_in)?;
        let token_out = Address::from_str(&request.token_out)?;

        let permit2_approval = self
            .get_tokens_to_approve(client.clone(), token_in, executor_address, PERMIT2_ADDRESS)
            .await?;

        let reactor_approval = self
            .get_tokens_to_approve(client.clone(), token_out, executor_address, REACTOR_ADDRESS)
            .await?;

        let execute_bytes = &route.method_parameters.calldata;
        let encoded_execute_bytes = hex::decode(&execute_bytes[2..]).expect("Failed to decode hex");

        // abi encode as [tokens to approve to swap router 02, tokens to approve to reactor,  multicall data]
        //               [address[], address[], bytes[]]
        let encoded_calldata = ethabi::encode(&[
            Token::Array(permit2_approval),
            Token::Array(reactor_approval),
            Token::Bytes(encoded_execute_bytes),
        ]);
        let orders: Vec<UniversalRouterExecutor::SignedOrder> = signed_orders
            .into_iter()
            .map(|order| UniversalRouterExecutor::SignedOrder {
                order: order.order,
                sig: order.sig,
            })
            .collect();
        let call = fill_contract.executeBatch(orders, Bytes::from(encoded_calldata));

        Ok(call.into_transaction_request().with_chain_id(chain_id))
    }

    fn current_timestamp(&self) -> Result<u64> {
        let start = SystemTime::now();
        Ok(start.duration_since(UNIX_EPOCH)?.as_secs())
    }

    async fn get_tokens_to_approve(
        &self,
        client: Arc<DynProvider<AnyNetwork>>,
        token: Address,
        from: &str,
        to: &str,
    ) -> Result<Vec<Token>, anyhow::Error> {
        if token == Address::ZERO {
            return Ok(vec![]);
        }
        let token_contract = ERC20::new(token, client.clone());
        let allowance = token_contract
            .allowance(
                Address::from_str(from).expect("Error encoding from address"),
                Address::from_str(to).expect("Error encoding from address"),
            )
            .call()
            .await
            .expect("Failed to get allowance");
        if allowance._0 < U256::MAX / U256::from(2) {
            Ok(vec![Token::Address(H160(token.0 .0))])
        } else {
            Ok(vec![])
        }
    }

    fn get_profit_eth(&self, RoutedOrder { request, route, .. }: &RoutedOrder) -> Option<U256> {
        let quote = U256::from_str_radix(&route.quote, 10).ok()?;
        let amount_required =
            U256::from_str_radix(&request.amount_required.to_string(), 10).ok()?;

        // exact_out: quote must be less than amount_in_required
        // exact_in: quote must be greater than amount_out_required
        if (request.orders.first().unwrap().order.is_exact_output() && quote.ge(&amount_required))
            || (!request.orders.first().unwrap().order.is_exact_output()
                && quote.le(&amount_required))
        {
            return None;
        }

        // exact_out: profit = amount_in_required - quote
        // exact_in: profit = quote - amount_out_required
        let profit_quote = if request.orders.first().unwrap().order.is_exact_output() {
            amount_required.saturating_sub(quote)
        } else {
            quote.saturating_sub(amount_required)
        };

        if request.token_out.to_lowercase() == WETH_ADDRESS.to_lowercase() {
            return Some(profit_quote);
        }

        let gas_use_eth = U256::from_str_radix(&route.gas_use_estimate, 10)
            .ok()?
            .saturating_mul(U256::from_str_radix(&route.gas_price_wei, 10).ok()?);
        profit_quote
            .saturating_mul(gas_use_eth)
            .checked_div(U256::from_str_radix(&route.gas_use_estimate_quote, 10).ok()?)
    }

    /// Converts the quote amount to ETH equivalent value
    ///
    /// For WETH output tokens, returns the quote directly since it's already in ETH.
    /// For non-WETH output tokens, converts using the following formula:
    /// quote_eth = quote * gas_wei / gas_in_quote
    ///
    /// # Arguments
    /// * `request` - The order request containing token information
    /// * `route` - The route containing quote and gas estimates
    ///
    /// # Returns
    /// * `Some(U256)` - The quote value in ETH
    /// * `None` - If any conversion fails or division by zero would occur
    fn get_quote_eth(&self, RoutedOrder { request, route, .. }: &RoutedOrder) -> Option<U256> {
        let quote = U256::from_str_radix(&route.quote, 10).ok()?;

        // If output token is WETH, quote is already in ETH
        if request.token_out.to_lowercase() == WETH_ADDRESS.to_lowercase() {
            return Some(quote);
        }

        let gas_use_eth = U256::from_str_radix(&route.gas_use_estimate, 10)
            .ok()?
            .saturating_mul(U256::from_str_radix(&route.gas_price_wei, 10).ok()?);
        quote
            .saturating_mul(gas_use_eth)
            .checked_div(U256::from_str_radix(&route.gas_use_estimate_quote, 10).ok()?)
    }

    /// Get the minimum gas price on Arbitrum
    /// https://docs.arbitrum.io/build-decentralized-apps/precompiles/reference#arbgasinfo
    async fn get_arbitrum_min_gas_price(
        &self,
        client: Arc<DynProvider<AnyNetwork>>,
    ) -> Result<U256> {
        let precompile_address = ARBITRUM_GAS_PRECOMPILE.parse::<Address>()?;
        let gas_precompile = GasPrecompileContract::new(precompile_address, client.clone());
        let gas_info = gas_precompile.getMinimumGasPrice().call().await?._0;

        Ok(gas_info)
    }
}
