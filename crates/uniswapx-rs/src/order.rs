use std::error::Error;

use alloy_dyn_abi::SolType;
use alloy_primitives::Uint;
use alloy_primitives::I256;
use alloy_primitives::U256;
use alloy_sol_types::sol;
use anyhow::Result;
use serde::Serialize;

use crate::sol_math::MulDiv;

fn current_timestamp_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System time should never be before Unix epoch")
        .as_millis() as u64
}

sol! {
    #[derive(Debug)]
    struct OrderInfo {
        address reactor;
        address swapper;
        uint256 nonce;
        uint256 deadline;
        address additionalValidationContract;
        bytes additionalValidationData;
    }

    #[derive(Debug)]
    struct DutchOutput {
        address token;
        uint256 startAmount;
        uint256 endAmount;
        address recipient;
    }

    #[derive(Debug)]
    struct DutchInput {
        address token;
        uint256 startAmount;
        uint256 endAmount;
    }

    #[derive(Debug)]
    struct CosignerData {
        uint256 decayStartTime;
        uint256 decayEndTime;
        address exclusiveFiller;
        uint256 exclusivityOverrideBps;
        uint256 inputAmount;
        uint256[] outputAmounts;
    }

    #[derive(Debug)]
    struct V2DutchOrder {
        OrderInfo info;
        address cosigner;
        DutchInput baseInput;
        DutchOutput[] baseOutputs;
        CosignerData cosignerData;
        bytes cosignature;
    }

    #[derive(Debug)]
    struct PriorityInput {
        address token;
        uint256 amount;
        uint256 mpsPerPriorityFeeWei;
    }

    #[derive(Debug)]
    struct PriorityOutput {
        address token;
        uint256 amount;
        uint256 mpsPerPriorityFeeWei;
        address recipient;
    }

    #[derive(Debug)]
    struct PriorityCosignerData {
        uint256 auctionTargetBlock;
    }

    #[derive(Debug)]
    struct PriorityOrder {
        OrderInfo info;
        address cosigner;
        uint256 auctionStartBlock;
        uint256 baselinePriorityFeeWei;
        PriorityInput input;
        PriorityOutput[] outputs;
        PriorityCosignerData cosignerData;
        bytes cosignature;
    }

    #[derive(Debug)]
    struct V3DutchOrder {
        OrderInfo info;
        address cosigner;
        uint256 startingBaseFee;
        V3DutchInput baseInput;
        V3DutchOutput[] baseOutputs;
        V3CosignerData cosignerData;
        bytes cosignature;
    }

    #[derive(Debug)]
    struct V3CosignerData {
        uint256 decayStartBlock;
        address exclusiveFiller;
        uint256 exclusivityOverrideBps;
        uint256 inputAmount;
        uint256[] outputAmounts;
    }

    #[derive(Debug)]
    struct NonlinearDutchDecay {
        uint256 relativeBlocks;
        int256[] relativeAmounts;
    }

    #[derive(Debug)]
    struct V3DutchInput {
        address token;
        uint256 startAmount;
        NonlinearDutchDecay curve;
        uint256 maxAmount;
        uint256 adjustmentPerGweiBaseFee;
    }
    
    #[derive(Debug)]
    struct V3DutchOutput {
        address token;
        uint256 startAmount;
        NonlinearDutchDecay curve;
        address recipient;
        uint256 minAmount;
        uint256 adjustmentPerGweiBaseFee;
    }

    /// @notice Input tokens for hybrid auction
    /// @dev if exact-in, input amount is fixed at maxAmount
    /// @dev if exact-out, scale down from maxAmount
    #[derive(Debug)]
    struct HybridInput {
        address token;
        uint256 maxAmount;
    }

    /// @notice Output tokens for hybrid auction
    /// @dev if exact-in, scale up from minAmount
    /// @dev if exact-out, output amount is fixed at minAmount
    #[derive(Debug)]
    struct HybridOutput {
        address token;
        uint256 minAmount;
        address recipient;
    }

    /// @notice Cosigner data for hybrid auction orders
    #[derive(Debug)]
    struct HybridCosignerData {
        uint256 auctionTargetBlock;
        uint256[] supplementalPriceCurve;
    }

    /// @notice Hybrid auction order combining Dutch decay and priority gas auctions
    #[derive(Debug)]
    struct HybridOrder {
        OrderInfo info;
        address cosigner;
        HybridInput input;
        HybridOutput[] outputs;
        uint256 auctionStartBlock;
        uint256 baselinePriorityFee;
        uint256 scalingFactor;
        uint256[] priceCurve;
        HybridCosignerData cosignerData;
        bytes cosignature;
    }
}

pub const MPS: u64 = 1e7 as u64;
pub const BPS: U256 = Uint::from_limbs([10000, 0, 0, 0]);
const PACKED_UINT16_ARRAY_LENGTH: usize = 256 / 16;

#[derive(Debug, Clone)]
pub enum Order {
    V2DutchOrder(V2DutchOrder),
    PriorityOrder(PriorityOrder),
    V3DutchOrder(V3DutchOrder),
    HybridOrder(HybridOrder),
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TradeType {
    #[serde(rename = "exactIn")]
    ExactIn,
    #[serde(rename = "exactOut")]
    ExactOut,
}

impl Order {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Order::V2DutchOrder(order) => order.encode_inner(),
            Order::PriorityOrder(order) => order.encode_inner(),
            Order::V3DutchOrder(order) => order.encode_inner(),
            Order::HybridOrder(order) => order.encode_inner(),
        }
    }

    pub fn trade_type(&self) -> TradeType {
        match self {
            Order::V2DutchOrder(order) => {
                if order.baseOutputs.iter().any(|o| o.startAmount == o.endAmount) {
                    TradeType::ExactOut
                } else {
                    TradeType::ExactIn
                }
            }
            Order::PriorityOrder(order) => {
                if order.outputs.iter().any(|o| o.mpsPerPriorityFeeWei == U256::from(0)) {
                    TradeType::ExactOut
                } else {
                    TradeType::ExactIn
                }
            }
            Order::V3DutchOrder(order) => {
                if order.baseOutputs.iter().any(
                    |o| o.curve.relativeAmounts.len() == 0 ||
                    o.curve.relativeAmounts.iter().all(|&x| x.eq(&I256::ZERO))
                ) {
                    TradeType::ExactOut
                } else {
                    TradeType::ExactIn
                }
            }
            Order::HybridOrder(order) => {
                // HybridOrder trade type is determined by scalingFactor
                // > 1e18 = exact-in, < 1e18 = exact-out, == 1e18 = depends on price curve
                if order.scalingFactor > BASE_SCALING_FACTOR {
                    TradeType::ExactIn
                } else if order.scalingFactor < BASE_SCALING_FACTOR {
                    TradeType::ExactOut
                } else {
                    // Neutral scaling factor - default to exact-in
                    TradeType::ExactIn
                }
            }
        }
    }

    pub fn is_exact_output(&self) -> bool {
        matches!(self.trade_type(), TradeType::ExactOut)
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedInput {
    pub token: String,
    pub amount: U256,
}

#[derive(Debug, Clone)]
pub struct ResolvedOutput {
    pub token: String,
    pub amount: U256,
    pub recipient: String,
}

#[derive(Debug, Clone)]
pub struct ResolvedOrder {
    pub input: ResolvedInput,
    pub outputs: Vec<ResolvedOutput>,
}

#[derive(Debug)]
pub enum OrderResolution {
    Resolved(ResolvedOrder),
    Expired,
    Invalid,
    NotFillableYet(ResolvedOrder),
}

impl V2DutchOrder {
    pub fn decode_inner(order_hex: &[u8], validate: bool) -> Result<Self, Box<dyn Error>> {
        Ok(V2DutchOrder::abi_decode(order_hex, validate)?)
    }

    pub fn encode_inner(&self) -> Vec<u8> {
        V2DutchOrder::abi_encode(self)
    }

    pub fn resolve(&self, timestamp: u64) -> OrderResolution {
        let timestamp = U256::from(timestamp);

        if self.info.deadline.lt(&timestamp) {
            return OrderResolution::Expired;
        };

        // resolve over the decay curve
        // TODO: apply cosigner logic

        let input = ResolvedInput {
            token: self.baseInput.token.to_string(),
            amount: resolve_decay(
                timestamp,
                self.cosignerData.decayStartTime,
                self.cosignerData.decayEndTime,
                self.baseInput.startAmount,
                self.baseInput.endAmount,
            ),
        };

        let outputs: Result<Vec<ResolvedOutput>> = self
            .baseOutputs
            .iter()
            .map(|output| {
                let mut amount = resolve_decay(
                    timestamp,
                    self.cosignerData.decayStartTime,
                    self.cosignerData.decayEndTime,
                    output.startAmount,
                    output.endAmount,
                );

                // add exclusivity override to amount
                if self.cosignerData.decayStartTime.gt(&timestamp) && !self.cosignerData.exclusiveFiller.is_zero() {
                    let exclusivity = self.cosignerData.exclusivityOverrideBps.checked_add(BPS).ok_or(anyhow::Error::msg("Overflow in exclusivity calculation"))?;
                    let exclusivity = exclusivity.checked_mul(amount).ok_or(anyhow::Error::msg("Overflow in exclusivity calculation"))?;
                    amount = exclusivity.checked_div(BPS).ok_or(anyhow::Error::msg("Division by zero in exclusivity calculation"))?;
                };

                Ok(ResolvedOutput {
                    token: output.token.to_string(),
                    amount,
                    recipient: output.recipient.to_string(),
                })
            })
            .collect();

        match outputs {
            Ok(resolved_outputs) => OrderResolution::Resolved(ResolvedOrder { input, outputs: resolved_outputs }),
            Err(_) => OrderResolution::Invalid
        }
    }
}

// Estimates the target block timestamp based on the current block timestamp and average block time
pub fn projected_target_block_ms(current_block: u64, target_block: u64, block_timestamp: u64, block_time_ms: u64) -> U256 {
    let blocks_until_target = U256::from(target_block).saturating_sub(U256::from(current_block));
    let time_until_target_ms = blocks_until_target.saturating_mul(U256::from(block_time_ms));
    U256::from(block_timestamp).saturating_mul(U256::from(1000)).saturating_add(time_until_target_ms)
}

impl PriorityOrder {
    pub fn decode_inner(order_hex: &[u8], validate: bool) -> Result<Self, Box<dyn Error>> {
        Ok(PriorityOrder::abi_decode(order_hex, validate)?)
    }

    pub fn encode_inner(&self) -> Vec<u8> {
        PriorityOrder::abi_encode(self)
    }

    pub fn resolve(&self, block_number: u64, block_timestamp: u64, block_time_ms: u64, priority_fee: U256, min_block_percentage_buffer: u64) -> OrderResolution {
        let block_time = block_time_ms / 1000;
        let next_block_timestamp = U256::from(block_timestamp) + U256::from(block_time);

        let input = self.input.scale(priority_fee);
        let outputs = self
            .outputs
            .iter()
            .map(|output| output.scale(priority_fee))
            .collect();

        let min_start_block = std::cmp::min(self.cosignerData.auctionTargetBlock, self.auctionStartBlock);

        let current_block = U256::from(block_number);
        if self.info.deadline.lt(&next_block_timestamp) || current_block >= min_start_block {
            return OrderResolution::Expired;
        };
        
        // If current timestamp is > BLOCK_TIME away from target
        // then not yet fillable
        let target_block_ms = projected_target_block_ms(
            block_number,
            min_start_block.try_into().unwrap(),
            block_timestamp,
            block_time_ms
        );
        let time_buffer_ms = block_time_ms * min_block_percentage_buffer / 100;
        if U256::from(current_timestamp_ms() + time_buffer_ms).lt(&target_block_ms) {
            return OrderResolution::NotFillableYet(ResolvedOrder { input, outputs });
        }

        OrderResolution::Resolved(ResolvedOrder { input, outputs })
    }
}

impl PriorityInput {
    pub fn scale(&self, priority_fee: U256) -> ResolvedInput {
        let amount = self.amount.wrapping_mul(U256::from(MPS).wrapping_add(priority_fee.wrapping_mul(self.mpsPerPriorityFeeWei))).wrapping_div(U256::from(MPS));
        ResolvedInput {
            token: self.token.to_string(),
            amount,
        }
    }
}

impl PriorityOutput {
    pub fn scale(&self, priority_fee: U256) -> ResolvedOutput {
        let amount = self.amount.wrapping_mul(U256::from(MPS).saturating_sub(priority_fee.wrapping_mul(self.mpsPerPriorityFeeWei))).wrapping_div(U256::from(MPS));
        ResolvedOutput {
            token: self.token.to_string(),
            amount,
            recipient: self.recipient.to_string(),
        }
    }
}

impl V3DutchOrder {
    pub fn decode_inner(order_hex: &[u8], validate: bool) -> Result<Self, Box<dyn Error>> {
        Ok(V3DutchOrder::abi_decode(order_hex, validate)?)
    }

    pub fn encode_inner(&self) -> Vec<u8> {
        V3DutchOrder::abi_encode(self)
    }

    pub fn resolve(&self, block_number: u64, timestamp: u64) -> OrderResolution {
        let timestamp = U256::from(timestamp);

        if self.info.deadline.lt(&timestamp) {
            return OrderResolution::Expired;
        };

        // resolve over the decay curve
        let input = ResolvedInput {
            token: self.baseInput.token.to_string(),
            amount: match self.baseInput.curve.decay(
                self.baseInput.startAmount,
                self.cosignerData.decayStartBlock,
                U256::from(block_number),
                U256::from(0),
                self.baseInput.maxAmount,
                NonlinearDutchDecay::v3_linear_input_decay
            ) {
                Ok(amount) => amount,
                Err(_) => return OrderResolution::Invalid,
            },
        };

        let outputs: Result<Vec<ResolvedOutput>> = self
            .baseOutputs
            .iter()
            .map(|output| {
                let mut amount = output.curve.decay(
                    output.startAmount,
                    self.cosignerData.decayStartBlock,
                    U256::from(block_number),
                    output.minAmount,
                    U256::MAX,
                    NonlinearDutchDecay::v3_linear_output_decay
                )?;
                
                // add exclusivity override to amount if before decay start block
                if self.cosignerData.decayStartBlock.gt(&U256::from(block_number)) && !self.cosignerData.exclusiveFiller.is_zero() {
                    let exclusivity = self.cosignerData.exclusivityOverrideBps.checked_add(BPS).ok_or(anyhow::Error::msg("Overflow in exclusivity calculation"))?;
                    let exclusivity = exclusivity.checked_mul(amount).ok_or(anyhow::Error::msg("Overflow in exclusivity calculation"))?;
                    amount = exclusivity.checked_div(BPS).ok_or(anyhow::Error::msg("Division by zero in exclusivity calculation"))?;
                };

                Ok(ResolvedOutput {
                    token: output.token.to_string(),
                    amount,
                    recipient: output.recipient.to_string(),
                })
            })
            .collect();

        match outputs {
            Ok(resolved_outputs) => OrderResolution::Resolved(ResolvedOrder { input, outputs: resolved_outputs }),
            Err(_) => OrderResolution::Invalid,
        }
    }
}

/// Base scaling factor (1e18) used as neutral point for hybrid auctions
pub const BASE_SCALING_FACTOR: U256 = Uint::from_limbs([1_000_000_000_000_000_000u64, 0, 0, 0]);

/// Price curve element: upper 16 bits = duration (blocks), lower 240 bits = scaling factor
#[derive(Debug, Clone, Copy)]
pub struct PriceCurveElement {
    pub duration: u16,
    pub scaling_factor: U256,
}

impl PriceCurveElement {
    /// Parse a price curve element from a U256
    /// Format: (duration << 240) | scalingFactor
    pub fn from_u256(value: U256) -> Self {
        // Extract upper 16 bits as duration
        let duration_u256: U256 = value >> 240;
        let duration: u16 = u16::try_from(duration_u256.min(U256::from(u16::MAX))).unwrap_or(u16::MAX);
        // Mask for lower 240 bits (30 bytes)
        let mask: U256 = (U256::from(1u64) << 240) - U256::from(1u64);
        let scaling_factor = value & mask;
        Self { duration, scaling_factor }
    }
}

impl HybridOrder {
    pub fn decode_inner(order_hex: &[u8], validate: bool) -> Result<Self, Box<dyn Error>> {
        Ok(HybridOrder::abi_decode(order_hex, validate)?)
    }

    pub fn encode_inner(&self) -> Vec<u8> {
        HybridOrder::abi_encode(self)
    }

    /// Resolve the hybrid order at the given block number and timestamp
    ///
    /// # Arguments
    /// * `block_number` - Current block number
    /// * `timestamp` - Current block timestamp
    /// * `priority_fee` - Priority fee above baseline (tx.gasprice - block.basefee - baselinePriorityFee)
    ///
    /// # Returns
    /// * `OrderResolution` - The resolved order or status
    pub fn resolve(&self, block_number: u64, timestamp: u64, priority_fee: U256) -> OrderResolution {
        let timestamp = U256::from(timestamp);

        // Check deadline
        if self.info.deadline.lt(&timestamp) {
            return OrderResolution::Expired;
        }

        // Determine effective auction target block
        let auction_target_block = if self.cosignerData.auctionTargetBlock != U256::ZERO {
            self.cosignerData.auctionTargetBlock
        } else {
            self.auctionStartBlock
        };

        let block_number_u256 = U256::from(block_number);

        // Check if auction has started
        if auction_target_block != U256::ZERO && block_number_u256 < auction_target_block {
            // Not fillable yet - return with base amounts
            let input = ResolvedInput {
                token: self.input.token.to_string(),
                amount: self.input.maxAmount,
            };
            let outputs: Vec<ResolvedOutput> = self.outputs.iter().map(|o| ResolvedOutput {
                token: o.token.to_string(),
                amount: o.minAmount,
                recipient: o.recipient.to_string(),
            }).collect();
            return OrderResolution::NotFillableYet(ResolvedOrder { input, outputs });
        }

        // Calculate current scaling factor from price curve
        let effective_price_curve = if !self.cosignerData.supplementalPriceCurve.is_empty() {
            // Apply supplemental price curve (simplified - just use it directly for now)
            // Full implementation would combine with base price curve
            &self.cosignerData.supplementalPriceCurve
        } else {
            &self.priceCurve
        };

        let current_scaling_factor = match Self::get_price_curve_scaling(
            effective_price_curve,
            auction_target_block,
            block_number_u256,
        ) {
            Ok(factor) => factor,
            Err(_) => return OrderResolution::Invalid,
        };

        // Validate scaling direction consistency
        if !shares_scaling_direction(self.scalingFactor, current_scaling_factor) {
            return OrderResolution::Invalid;
        }

        // Determine if exact-in or exact-out mode
        let use_exact_in = self.scalingFactor > BASE_SCALING_FACTOR
            || (self.scalingFactor == BASE_SCALING_FACTOR && current_scaling_factor >= BASE_SCALING_FACTOR);

        // Calculate scaling multiplier with priority fee adjustment
        let scaling_multiplier = if use_exact_in {
            // Exact-in: scalingMultiplier = currentScalingFactor + ((scalingFactor - 1e18) * priorityFee)
            let priority_adjustment = self.scalingFactor
                .saturating_sub(BASE_SCALING_FACTOR)
                .saturating_mul(priority_fee);
            current_scaling_factor.saturating_add(priority_adjustment)
        } else {
            // Exact-out: scalingMultiplier = currentScalingFactor - ((1e18 - scalingFactor) * priorityFee)
            let priority_adjustment = BASE_SCALING_FACTOR
                .saturating_sub(self.scalingFactor)
                .saturating_mul(priority_fee);
            current_scaling_factor.saturating_sub(priority_adjustment)
        };

        // Resolve amounts based on mode
        if use_exact_in {
            // Exact-in: input is fixed at maxAmount, outputs are scaled up
            let input = ResolvedInput {
                token: self.input.token.to_string(),
                amount: self.input.maxAmount,
            };
            let outputs: Vec<ResolvedOutput> = self.outputs.iter().map(|o| {
                // mulWadUp: (amount * scaling + WAD - 1) / WAD
                let scaled_amount = mul_wad_up(o.minAmount, scaling_multiplier);
                ResolvedOutput {
                    token: o.token.to_string(),
                    amount: scaled_amount,
                    recipient: o.recipient.to_string(),
                }
            }).collect();
            OrderResolution::Resolved(ResolvedOrder { input, outputs })
        } else {
            // Exact-out: outputs are fixed at minAmount, input is scaled down
            let scaled_input = mul_wad(self.input.maxAmount, scaling_multiplier);
            let input = ResolvedInput {
                token: self.input.token.to_string(),
                amount: scaled_input,
            };
            let outputs: Vec<ResolvedOutput> = self.outputs.iter().map(|o| ResolvedOutput {
                token: o.token.to_string(),
                amount: o.minAmount,
                recipient: o.recipient.to_string(),
            }).collect();
            OrderResolution::Resolved(ResolvedOrder { input, outputs })
        }
    }

    /// Calculate the current scaling factor from the price curve
    ///
    /// # Arguments
    /// * `price_curve` - Array of price curve elements
    /// * `target_block` - The auction start block
    /// * `fill_block` - The current block number
    ///
    /// # Returns
    /// * `Result<U256>` - The current scaling factor or error
    fn get_price_curve_scaling(
        price_curve: &[U256],
        target_block: U256,
        fill_block: U256,
    ) -> Result<U256> {
        // Empty price curve returns neutral scaling
        if price_curve.is_empty() {
            return Ok(BASE_SCALING_FACTOR);
        }

        // No auction (target_block == 0) with price curve is invalid
        if target_block == U256::ZERO {
            return Err(anyhow::anyhow!("Invalid target block designation"));
        }

        // Calculate blocks passed since target
        if fill_block < target_block {
            return Err(anyhow::anyhow!("Fill block before target block"));
        }
        let blocks_passed = fill_block.saturating_sub(target_block);

        Self::get_calculated_values(price_curve, blocks_passed)
    }

    /// Calculate the scaling factor value based on block progression through the price curve
    /// This mirrors PriceCurveLib.getCalculatedValues from Solidity
    fn get_calculated_values(parameters: &[U256], blocks_passed: U256) -> Result<U256> {
        if parameters.is_empty() {
            return Ok(BASE_SCALING_FACTOR);
        }

        let mut blocks_counted = U256::ZERO;
        let mut current_scaling_factor = BASE_SCALING_FACTOR;
        let mut has_passed_zero_duration = false;

        for i in 0..parameters.len() {
            let element = PriceCurveElement::from_u256(parameters[i]);
            let duration = U256::from(element.duration);
            let scaling_factor = element.scaling_factor;

            // Special handling for zero duration
            if duration == U256::ZERO {
                if blocks_passed >= blocks_counted {
                    current_scaling_factor = scaling_factor;
                    has_passed_zero_duration = true;

                    // If exactly at this point, return these values
                    if blocks_passed == blocks_counted {
                        return Ok(scaling_factor);
                    }
                }
                continue;
            }

            // If blocks_passed is in this segment
            if blocks_passed < blocks_counted + duration {
                if has_passed_zero_duration && i > 0 {
                    let prev_element = PriceCurveElement::from_u256(parameters[i - 1]);
                    if prev_element.duration == 0 {
                        // Interpolate from zero duration values
                        let zero_duration_scaling = prev_element.scaling_factor;

                        if !shares_scaling_direction(zero_duration_scaling, scaling_factor) {
                            return Err(anyhow::anyhow!("Invalid price curve parameters"));
                        }

                        current_scaling_factor = locate_current_amount(
                            zero_duration_scaling,
                            scaling_factor,
                            blocks_counted,
                            blocks_passed,
                            blocks_counted + duration,
                            zero_duration_scaling > BASE_SCALING_FACTOR,
                        );
                        return Ok(current_scaling_factor);
                    }
                }

                // Standard interpolation
                let end_scaling_factor = if i + 1 < parameters.len() {
                    let next_element = PriceCurveElement::from_u256(parameters[i + 1]);
                    next_element.scaling_factor
                } else {
                    BASE_SCALING_FACTOR
                };

                if !shares_scaling_direction(scaling_factor, end_scaling_factor) {
                    return Err(anyhow::anyhow!("Invalid price curve parameters"));
                }

                current_scaling_factor = locate_current_amount(
                    scaling_factor,
                    end_scaling_factor,
                    blocks_counted,
                    blocks_passed,
                    blocks_counted + duration,
                    scaling_factor > BASE_SCALING_FACTOR,
                );
                return Ok(current_scaling_factor);
            }

            blocks_counted += duration;
        }

        // Exceeded total blocks
        if blocks_passed >= blocks_counted {
            return Err(anyhow::anyhow!("Price curve blocks exceeded"));
        }

        Ok(current_scaling_factor)
    }
}

/// Check if two values share the same scaling direction (both >= 1e18 or both <= 1e18)
fn shares_scaling_direction(a: U256, b: U256) -> bool {
    a == BASE_SCALING_FACTOR
        || b == BASE_SCALING_FACTOR
        || (a > BASE_SCALING_FACTOR) == (b > BASE_SCALING_FACTOR)
}

/// Linear interpolation between two amounts based on block progression
fn locate_current_amount(
    start_amount: U256,
    end_amount: U256,
    start_block: U256,
    current_block: U256,
    end_block: U256,
    round_up: bool,
) -> U256 {
    if start_amount == end_amount {
        return start_amount;
    }

    let duration = end_block.saturating_sub(start_block);
    let elapsed = current_block.saturating_sub(start_block);
    let remaining = duration.saturating_sub(elapsed);

    if duration == U256::ZERO {
        return start_amount;
    }

    // Calculate: (startAmount * remaining + endAmount * elapsed) / duration
    let start_weighted = start_amount.saturating_mul(remaining);
    let end_weighted = end_amount.saturating_mul(elapsed);
    let total = start_weighted.saturating_add(end_weighted);

    if round_up && total != U256::ZERO {
        // Round up: (total + duration - 1) / duration
        total.saturating_add(duration).saturating_sub(U256::from(1u64)) / duration
    } else {
        total / duration
    }
}

/// Multiply with WAD (1e18) precision, rounding down
/// result = (a * b) / 1e18
fn mul_wad(a: U256, b: U256) -> U256 {
    a.saturating_mul(b) / BASE_SCALING_FACTOR
}

/// Multiply with WAD (1e18) precision, rounding up
/// result = (a * b + 1e18 - 1) / 1e18
fn mul_wad_up(a: U256, b: U256) -> U256 {
    let product = a.saturating_mul(b);
    if product == U256::ZERO {
        U256::ZERO
    } else {
        product.saturating_add(BASE_SCALING_FACTOR).saturating_sub(U256::from(1u64)) / BASE_SCALING_FACTOR
    }
}

fn resolve_decay(
    at_time: U256,
    start_time: U256,
    end_time: U256,
    start_amount: U256,
    end_amount: U256,
) -> U256 {
    if end_time.le(&at_time) {
        return end_amount;
    }

    if at_time.le(&start_time) {
        return start_amount;
    }

    if end_time.eq(&start_time) {
        return start_amount;
    }

    if start_amount.eq(&end_amount) {
        return start_amount;
    }

    let duration = end_time.saturating_sub(start_time);
    let elapsed = at_time.saturating_sub(start_time);

    if start_amount.gt(&end_amount) {
        // decaying downward
        let decay = start_amount
            .saturating_sub(end_amount)
            .saturating_mul(elapsed)
            .checked_div(duration)
            .unwrap_or(U256::ZERO);
        return start_amount.saturating_sub(decay);
    } else {
        // decaying upward
        let decay = end_amount
            .saturating_sub(start_amount)
            .saturating_mul(elapsed)
            .checked_div(duration)
            .unwrap_or(U256::ZERO);
        return start_amount.saturating_add(decay);
    }
}

impl NonlinearDutchDecay {

    pub fn decay(
        &self,
        start_amount: U256,
        decay_start_block: U256,
        block_numberish: U256,
        min_amount: U256,
        max_amount: U256,
        decay_func: fn(U256, U256, U256, I256, I256) -> Result<I256>
    ) -> Result<U256> {
        // Check for invalid decay curve
        if self.relativeAmounts.len() > PACKED_UINT16_ARRAY_LENGTH {
            return Err(anyhow::anyhow!("Invalid decay curve"));
        }

        // Handle current block before decay or no decay
        if decay_start_block >= block_numberish || self.relativeAmounts.is_empty() {
            return Ok(start_amount.clamp(min_amount, max_amount));
        }

        // Cap block_delta to u16::MAX to prevent overflow
        let block_delta: u16 = u16::try_from(
            (block_numberish - decay_start_block).min(U256::from(u16::MAX))
        )?;

        let (start_point, end_point, rel_start_amount, rel_end_amount) = 
            self.locate_curve_position(block_delta)?;

        // Calculate decay of only the relative amounts
        let curve_delta = (decay_func)(
            U256::from(start_point),
            U256::from(end_point),
            U256::from(block_delta),
            rel_start_amount,
            rel_end_amount,
        )?;

        // Apply curve_delta to start_amount and bound the result
        let result = if curve_delta.is_negative() {
            start_amount.saturating_add(curve_delta.abs().try_into()?)
        } else {
            start_amount.saturating_sub(curve_delta.try_into()?)
        };

        Ok(result.clamp(min_amount, max_amount))
    }

    /// Returns the linear interpolation between two points for input decay
    ///
    /// # Arguments
    ///
    /// * `start_point` - The start of the decay
    /// * `end_point` - The end of the decay
    /// * `current_point` - The current position in the decay
    /// * `start_amount` - The amount at the start of the decay
    /// * `end_amount` - The amount at the end of the decay
    ///
    /// # Returns
    ///
    /// The interpolated amount as an I256
    pub fn v3_linear_input_decay(
        start_point: U256,
        end_point: U256,
        current_point: U256,
        start_amount: I256,
        end_amount: I256,
    ) -> Result<I256> {
        if current_point >= end_point {
            return Ok(end_amount);
        }
        let elapsed = current_point.saturating_sub(start_point);
        let duration = end_point.saturating_sub(start_point);
        let delta: I256;

        // Because start_amount + delta is subtracted from the original amount,
        // we want to maximize start_amount + delta to favor the swapper
        if end_amount < start_amount {
            delta = -(I256::try_from(
                U256::try_from(start_amount.checked_sub(end_amount)
                    .ok_or_else(|| anyhow::anyhow!("Underflow in start_amount - end_amount"))?)?
                    .mul_div_down(elapsed, duration)
                    .map_err(|e| anyhow::anyhow!("MulDivDown error: {}", e))?
            )?);
        } else {
            delta = I256::try_from(
                U256::try_from(end_amount.checked_sub(start_amount)
                .ok_or_else(|| anyhow::anyhow!("Underflow in end_amount - start_amount"))?)?
                .mul_div_up(elapsed, duration)
                .map_err(|e| anyhow::anyhow!("MulDivUp error: {}", e))?
            )?;
        }

        Ok(start_amount.saturating_add(delta))
    }

    /// Returns the linear interpolation between two points for output decay
    ///
    /// # Arguments
    ///
    /// * `start_point` - The start of the decay
    /// * `end_point` - The end of the decay
    /// * `current_point` - The current position in the decay
    /// * `start_amount` - The amount at the start of the decay
    /// * `end_amount` - The amount at the end of the decay
    ///
    /// # Returns
    ///
    /// The interpolated amount as an I256
    pub fn v3_linear_output_decay(
        start_point: U256,
        end_point: U256,
        current_point: U256,
        start_amount: I256,
        end_amount: I256,
    ) -> Result<I256> {
        if current_point >= end_point {
            return Ok(end_amount);
        }
        let elapsed = current_point.saturating_sub(start_point);
        let duration = end_point.saturating_sub(start_point);
        let delta: I256;

        // For outputs, we want to minimize start_amount + delta to favor the swapper
        if end_amount < start_amount {
            delta = -(I256::try_from(
                U256::try_from(start_amount.checked_sub(end_amount)
                    .ok_or_else(|| anyhow::anyhow!("Underflow in start_amount - end_amount"))?)?
                    .mul_div_up(elapsed, duration)
                    .map_err(|e| anyhow::anyhow!("MulDivUp error: {}", e))?
            )?);
        } else {
            delta = I256::try_from(
                U256::try_from(end_amount.checked_sub(start_amount)
                .ok_or_else(|| anyhow::anyhow!("Underflow in end_amount - start_amount"))?)?
                .mul_div_down(elapsed, duration)
                .map_err(|e| anyhow::anyhow!("MulDivDown error: {}", e))?
            )?;
        }

        Ok(start_amount.saturating_add(delta))
    }

    /// Locates the position on the decay curve based on the current block
    fn locate_curve_position(&self, current_relative_block: u16) -> Result<(u16, u16, I256, I256)> {
        // Position is before the start of the curve
        if Self::get_element(self.relativeBlocks, 0)? >= current_relative_block {
            return Ok((0, Self::get_element(self.relativeBlocks, 0)?, I256::ZERO, self.relativeAmounts[0]));
        }
        let last_curve_index = self.relativeAmounts.len() - 1;
        for i in 1..=last_curve_index {
            if Self::get_element(self.relativeBlocks, i)? >= current_relative_block {
                return Ok(
                    (
                        Self::get_element(self.relativeBlocks, i - 1)?,
                        Self::get_element(self.relativeBlocks, i)?,
                        self.relativeAmounts[i - 1],
                        self.relativeAmounts[i],
                    )
                );
            }
        }

        Ok(
            (
                Self::get_element(self.relativeBlocks, last_curve_index)?,
                Self::get_element(self.relativeBlocks, last_curve_index)?,
                self.relativeAmounts[last_curve_index],
                self.relativeAmounts[last_curve_index],
            )
        )
    }

    /// Convert a u16 array into a single Uint<256, 4> value
    /// 
    /// This function packs up to 16 u16 values into a single Uint<256, 4>.
    /// Each u16 value occupies 16 bits in the resulting Uint.
    /// 
    /// # Arguments
    /// 
    /// * `input_array` - A slice of u16 values to be packed
    /// 
    /// # Returns
    /// 
    /// * `Result<Uint<256, 4>>` - The packed Uint value or an error
    pub fn to_uint16_array(input_array: &[u16]) -> Result<U256> {
        if input_array.len() > PACKED_UINT16_ARRAY_LENGTH {
            return Err(anyhow::Error::msg("Invalid array length"));
        }

        let mut packed_data = U256::ZERO;

        for (i, &value) in input_array.iter().enumerate() {
            let shifted_value = U256::from(value) << (i * 16);
            packed_data |= shifted_value;
        }

        Ok(packed_data)
    }

    
    /// Retrieve the nth uint16 value from a packed uint256
    fn get_element(packed_data: U256, n: usize) -> Result<u16> {
        if n >= PACKED_UINT16_ARRAY_LENGTH {
            return Err(anyhow::Error::msg("IndexOutOfBounds"));
        }
        
        let shift_amount = n * 16;
        let masked_value = (packed_data >> shift_amount) & U256::from(0xFFFF);
        let result = u16::try_from(masked_value)?;
        Ok(result)
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    const DECAY_FUNCTIONS: [fn(U256, U256, U256, I256, I256) -> Result<I256>; 2] = [
        NonlinearDutchDecay::v3_linear_input_decay,
        NonlinearDutchDecay::v3_linear_output_decay
    ];

    #[test]
    fn test_decay_after_end_time() {
        let start_time = U256::from(1);
        let end_time = U256::from(10);
        let start_amount = U256::from(100000);
        let end_amount = U256::from(100000000);

        let at_time = U256::from(11);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, end_amount);
    }

    #[test]
    fn test_decay_at_end_time() {
        let start_time = U256::from(1);
        let end_time = U256::from(10);
        let start_amount = U256::from(100000);
        let end_amount = U256::from(100000000);

        let at_time = U256::from(10);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, end_amount);
    }

    #[test]
    fn test_decay_before_start_time() {
        let start_time = U256::from(10);
        let end_time = U256::from(100);
        let start_amount = U256::from(100000);
        let end_amount = U256::from(100000000);

        let at_time = U256::from(5);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, start_amount);
    }

    #[test]
    fn test_decay_at_start_time() {
        let start_time = U256::from(10);
        let end_time = U256::from(100);
        let start_amount = U256::from(100000);
        let end_amount = U256::from(100000000);

        let at_time = U256::from(10);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, start_amount);
    }

    #[test]
    fn test_upwards_decay() {
        let start_time = U256::from(10);
        let end_time = U256::from(20);
        let start_amount = U256::from(100000);
        let end_amount = U256::from(200000);

        let at_time = U256::from(15);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, U256::from(150000));
    }

    #[test]
    fn test_downwards_decay() {
        let start_time = U256::from(10);
        let end_time = U256::from(20);
        let start_amount = U256::from(200000);
        let end_amount = U256::from(100000);

        let at_time = U256::from(15);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, U256::from(150000));
    }

    #[test]
    fn test_nonlinear_decay_before_start() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(999);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );
            assert_eq!(result.unwrap(), start_amount);
        }
    }

    #[test]
    fn test_nonlinear_decay_at_start() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1000);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );

            assert_eq!(result.unwrap(), U256::from(1000));
        }
    }

    #[test]
    fn test_nonlinear_decay_midway() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1150);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );

            assert_eq!(result.unwrap(), U256::from(100));
        }
    }

    #[test]
    fn test_nonlinear_decay_at_end() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1500);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );

            assert_eq!(result.unwrap(), U256::from(800));
        }
    }

    #[test]
    fn test_nonlinear_decay_after_end() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1600);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );
        assert_eq!(result.unwrap(), U256::from(800));
        }
    }

    #[test]
    fn test_nonlinear_decay_with_min_amount() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1100);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(300);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );

            assert_eq!(result.unwrap(), min_amount);
        }
    }

    #[test]
    fn test_nonlinear_decay_with_max_amount() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![
                100,
                200,
                300,
                400,
                500,
            ]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
                I256::try_from(600).unwrap(),
                I256::try_from(400).unwrap(),
                I256::try_from(200).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1500);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::from(500);

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );
            assert_eq!(result.unwrap(), max_amount);
        }
    }

    #[test]
    fn test_nonlinear_decay_start_amount_underflow() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![100, 200]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(1000).unwrap(),
                I256::try_from(800).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1100);
        let start_amount = U256::from(500); // Less than max relativeAmount
        let min_amount = U256::from(10);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );
            // Cannot fall below min_amount, even upon underflow
            assert_eq!(result.unwrap(), min_amount);
        }
    }

    #[test]
    fn test_nonlinear_decay_start_amount_overflow() {
        let decay = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![100, 200]).unwrap(),
            relativeAmounts: vec![
                I256::try_from(-1000).unwrap(),
                I256::try_from(-800).unwrap(),
            ],
        };

        let start_block = U256::from(1000);
        let current_block = U256::from(1100);
        let start_amount = U256::MAX;
        let min_amount = U256::from(10);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );
            // Cannot go above max_amount, even upon overflow
            assert_eq!(result.unwrap(), max_amount);
        }
    }

    #[test]
    fn test_nonlinear_decay_relative_blocks_too_long() {
        // Attempt to create the packed relativeBlocks
        let relative_blocks = NonlinearDutchDecay::to_uint16_array(&vec![
            100, 200, 300, 400, 500, 600, 700, 800, 900, 1000,
            1100, 1200, 1300, 1400, 1500, 1600, 1700
        ]);

        // Ensure that to_uint16_array returned an error due to excessive length
        assert!(relative_blocks.is_err());

        // Optionally, you can check the error message
        if let Err(e) = relative_blocks {
            assert_eq!(e.to_string(), "Invalid array length");
        }
    }

    #[test]
    fn test_nonlinear_decay_empty_inputs() {
        let decay_empty = NonlinearDutchDecay {
            relativeBlocks: NonlinearDutchDecay::to_uint16_array(&vec![]).unwrap(),
            relativeAmounts: vec![],
        };
        let start_block = U256::from(1000);
        let current_block = U256::from(1100);
        let start_amount = U256::from(1000);
        let min_amount = U256::from(0);
        let max_amount = U256::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay_empty.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), start_amount);
        }
    }
}
