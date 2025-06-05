use crate::collectors::uniswapx_route_collector::{Route, RoutedOrder};
use crate::shared::get_reactor_address;
use alloy::{
    network::{AnyNetwork, TransactionBuilder},
    primitives::{Address, U256},
    providers::{DynProvider, Provider},
    rpc::types::TransactionRequest,
    serde::WithOtherFields,
    sol,
    sol_types::SolCall,
};
use alloy_primitives::Bytes;
use anyhow::Result;
use async_trait::async_trait;
use bindings_uniswapx::{
    basereactor::BaseReactor::SignedOrder, erc20::ERC20, swaprouter02executor::SwapRouter02Executor,
};
use ethabi::ethereum_types::H160;
use std::{
    str::FromStr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use tracing::info;

use ethers::abi::Token;

const PERMIT2_ADDRESS: &str = "0x000000000022D473030F116dDEE9F6B43aC78BA3";
pub const WETH_ADDRESS: &str = "0x1514000000000000000000000000000000000000";
const ARBITRUM_GAS_PRECOMPILE: &str = "0x000000000000000000000000000000000000006C";

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract GasPrecompileContract {
        function getMinimumGasPrice() external view returns (uint256);
    }
}

sol! {
    enum PoolType {
        V2Pool,
        V3Pool
    }

    struct SwapRoute {
        address routerAddress;
        PoolType poolType;
        address tokenIn;
        address tokenOut;
        uint24 fee;
    }

    struct ExactInputParams {
        SwapRoute[] swapRoutes;
        address recipient;
        uint256 deadline;
        uint256 amountIn;
        uint256 amountOutMinimum;
    }

    function swapMultiroutes(ExactInputParams[] params) payable;
}

#[async_trait]
pub trait UniswapXStrategy {
    // builds a transaction to fill an order
    async fn build_fill(
        &self,
        client: Arc<DynProvider<AnyNetwork>>,
        executor_address: &str,
        signed_orders: Vec<SignedOrder>,
        route_order: &RoutedOrder,
    ) -> Result<WithOtherFields<TransactionRequest>> {
        let request = route_order.request.clone();
        let chain_id = client.get_chain_id().await?;
        let fill_contract =
            SwapRouter02Executor::new(Address::from_str(executor_address)?, client.clone());

        info!("executor_address: {}", executor_address);

        let token_in = Address::from_str(&request.token_in)?;
        let token_out = Address::from_str(&request.token_out)?;

        let permit2_approval = self
            .get_tokens_to_approve(client.clone(), token_in, executor_address, PERMIT2_ADDRESS)
            .await?;

        let reactor_approval = self
            .get_tokens_to_approve(
                client.clone(),
                token_out,
                executor_address,
                &get_reactor_address("default"),
            )
            .await?;

        let encoded_execute_bytes = self
            .encode_multiroute_calldata(route_order, executor_address)
            .await;

        // abi encode as [tokens to approve to swap router 02, tokens to approve to reactor,  multicall data]
        //               [address[], address[], bytes[]]
        let encoded_calldata = ethabi::encode(&[
            Token::Array(permit2_approval),
            Token::Array(reactor_approval),
            Token::Bytes(encoded_execute_bytes),
        ]);

        let orders: Vec<SwapRouter02Executor::SignedOrder> = signed_orders
            .into_iter()
            .map(|order| SwapRouter02Executor::SignedOrder {
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
        info!("allowance: {:?}", allowance._0);
        if allowance._0 < U256::MAX / U256::from(2) {
            Ok(vec![Token::Address(H160(token.0 .0))])
        } else {
            Ok(vec![])
        }
    }

    fn get_profit_eth(&self, RoutedOrder { request, route, .. }: &RoutedOrder) -> Option<U256> {
        let quote = U256::from_str_radix(&route.quote, 10).ok()?;

        info!("quote: {}", quote);

        let amount_required =
            U256::from_str_radix(&request.amount_required.to_string(), 10).ok()?;

        info!("amount_required: {}", amount_required);

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

        info!("profit_quote: {}", profit_quote);

        if request.token_out.to_lowercase() == WETH_ADDRESS.to_lowercase() {
            return Some(profit_quote);
        }

        let gas_use_eth = U256::from_str_radix(&route.gas_use_estimate, 10)
            .ok()?
            .saturating_mul(U256::from_str_radix(&route.gas_price_wei, 10).ok()?);
        info!("gas_use_eth: {}", gas_use_eth);
        info!(
            "gas_use_estimate_quote: {}",
            U256::from_str_radix(&route.gas_use_estimate_quote, 10).ok()?
        );
        info!(
            "profit_quote * gas_use_eth: {}",
            profit_quote.saturating_mul(gas_use_eth)
        );
        info!(
            "profit_quote * gas_use_eth / gas_use_estimate_quote: {:?}",
            profit_quote
                .saturating_mul(gas_use_eth)
                .checked_div(U256::from_str_radix(&route.gas_use_estimate_quote, 10).ok()?)
        );
        profit_quote.saturating_mul(gas_use_eth).checked_div(
            U256::from_str_radix(&route.gas_use_estimate_quote, 10)
                .ok()
                .filter(|&x| x != U256::ZERO)
                .unwrap_or(U256::from(100_000)), // Default to 100k gas if 0 or invalid
        )
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

    async fn encode_multiroute_calldata(
        &self,
        route_order: &RoutedOrder,
        executor_address: &str,
    ) -> Vec<u8> {
        // encode array of ExactInputParams
        let mut params = Vec::new();
        for path in route_order.route.route.iter() {
            if path.is_empty() {
                continue;
            }

            let mut swap_routes = Vec::new();

            // Build SwapRoute for each route in the path
            for route in path.iter() {
                let (pool_type, fee, router_addr, token_in_addr, token_out_addr) = match route {
                    Route::V2(v2_route) => (
                        PoolType::V2Pool,
                        0u32,
                        &v2_route.router_address,
                        &v2_route.token_in.address,
                        &v2_route.token_out.address,
                    ),
                    Route::V3(v3_route) => {
                        let fee_val = v3_route.fee.parse::<u32>().unwrap_or(3000);
                        (
                            PoolType::V3Pool,
                            fee_val,
                            &v3_route.router_address,
                            &v3_route.token_in.address,
                            &v3_route.token_out.address,
                        )
                    }
                    Route::V3S1(v3s1_route) => {
                        let fee_val = v3s1_route.fee.parse::<u32>().unwrap_or(500);
                        (
                            PoolType::V3Pool,
                            fee_val,
                            &v3s1_route.router_address,
                            &v3s1_route.token_in.address,
                            &v3s1_route.token_out.address,
                        )
                    }
                };

                let swap_route = SwapRoute {
                    routerAddress: Address::from_str(router_addr).unwrap_or(Address::ZERO),
                    poolType: pool_type,
                    tokenIn: Address::from_str(token_in_addr).unwrap_or(Address::ZERO),
                    tokenOut: Address::from_str(token_out_addr).unwrap_or(Address::ZERO),
                    fee: alloy_primitives::Uint::<24, 1>::from(fee),
                };
                swap_routes.push(swap_route);
            }

            // Get amount_in from the first element of path
            let amount_in = if let Some(first_route) = path.first() {
                match first_route {
                    Route::V2(v2_route) => v2_route.amount_in.as_ref(),
                    Route::V3(v3_route) => v3_route.amount_in.as_ref(),
                    Route::V3S1(v3s1_route) => v3s1_route.amount_in.as_ref(),
                }
                .and_then(|amount_str| U256::from_str_radix(amount_str, 10).ok())
                .unwrap_or(U256::ZERO)
            } else {
                U256::ZERO
            };

            // Get amount_out from the last element of path for calculating minimum
            let amount_out = if let Some(last_route) = path.last() {
                match last_route {
                    Route::V2(v2_route) => v2_route.amount_out.as_ref(),
                    Route::V3(v3_route) => v3_route.amount_out.as_ref(),
                    Route::V3S1(v3s1_route) => v3s1_route.amount_out.as_ref(),
                }
                .and_then(|amount_str| U256::from_str_radix(amount_str, 10).ok())
                .unwrap_or(amount_in) // Fallback to amount_in if no amount_out
            } else {
                amount_in
            };

            // Calculate amount_out_min with 3% slippage
            let slippage_factor = U256::from(97);
            let hundred = U256::from(100);
            let amount_out_min = amount_out * slippage_factor / hundred;

            let p = ExactInputParams {
                swapRoutes: swap_routes,
                recipient: Address::from_str(executor_address).unwrap_or(Address::ZERO),
                deadline: U256::from(
                    route_order
                        .target_block
                        .unwrap_or(U256::from(1234567890u64)),
                ),
                amountIn: amount_in,
                amountOutMinimum: amount_out_min,
            };

            params.push(p);
        }

        let call = swapMultiroutesCall { params };
        let mut out = Vec::with_capacity(call.abi_encoded_size());
        call.abi_encode_raw(&mut out);
        out
    }
}
