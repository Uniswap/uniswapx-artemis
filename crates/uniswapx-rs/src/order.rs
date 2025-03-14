use std::error::Error;

use alloy_dyn_abi::SolType;
use alloy_primitives::Uint;
use alloy_primitives::I256;
use alloy_sol_types::sol;
use anyhow::Result;

use crate::sol_math::MulDiv;

type BigUint = Uint<256, 4>;

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
}

pub const MPS: u64 = 1e7 as u64;
pub const BPS: BigUint = Uint::from_limbs([10000, 0, 0, 0]);
const PACKED_UINT16_ARRAY_LENGTH: usize = 256 / 16;

#[derive(Debug, Clone)]
pub enum Order {
    V2DutchOrder(V2DutchOrder),
    PriorityOrder(PriorityOrder),
    V3DutchOrder(V3DutchOrder),
}

impl Order {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Order::V2DutchOrder(order) => order.encode_inner(),
            Order::PriorityOrder(order) => order.encode_inner(),
            Order::V3DutchOrder(order) => order.encode_inner(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedInput {
    pub token: String,
    pub amount: BigUint,
}

#[derive(Debug, Clone)]
pub struct ResolvedOutput {
    pub token: String,
    pub amount: BigUint,
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
        let timestamp = BigUint::from(timestamp);

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

impl PriorityOrder {
    pub fn decode_inner(order_hex: &[u8], validate: bool) -> Result<Self, Box<dyn Error>> {
        Ok(PriorityOrder::abi_decode(order_hex, validate)?)
    }

    pub fn encode_inner(&self) -> Vec<u8> {
        PriorityOrder::abi_encode(self)
    }

    pub fn resolve(&self, block_number: u64, timestamp: u64, priority_fee: BigUint, has_calldata: bool) -> OrderResolution {
        let timestamp = BigUint::from(timestamp);

        if self.info.deadline.lt(&timestamp) {
            return OrderResolution::Expired;
        };

        let input = self.input.scale(priority_fee);
        let outputs = self
            .outputs
            .iter()
            .map(|output| output.scale(priority_fee))
            .collect();

        let min_start_block = std::cmp::min(self.cosignerData.auctionTargetBlock, self.auctionStartBlock);

        // If the order already has calldata, we can quickly process it the block before the target block
        // Otherwise, we need to process it in targetBlock - 2 to give time for the routing-api call (~1 second)
        let buffer = if has_calldata { 1 } else { 2 };
        if BigUint::from(block_number).lt(&min_start_block.saturating_sub(BigUint::from(buffer))) {
            return OrderResolution::NotFillableYet(ResolvedOrder { input, outputs });
        };

        OrderResolution::Resolved(ResolvedOrder { input, outputs })
    }
}

impl PriorityInput {
    pub fn scale(&self, priority_fee: BigUint) -> ResolvedInput {
        let amount = self.amount.wrapping_mul(BigUint::from(MPS).wrapping_add(priority_fee.wrapping_mul(self.mpsPerPriorityFeeWei))).wrapping_div(BigUint::from(MPS));
        ResolvedInput {
            token: self.token.to_string(),
            amount,
        }
    }
}

impl PriorityOutput {
    pub fn scale(&self, priority_fee: BigUint) -> ResolvedOutput {
        let amount = self.amount.wrapping_mul(BigUint::from(MPS).saturating_sub(priority_fee.wrapping_mul(self.mpsPerPriorityFeeWei))).wrapping_div(BigUint::from(MPS));
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
        let timestamp = BigUint::from(timestamp);

        if self.info.deadline.lt(&timestamp) {
            return OrderResolution::Expired;
        };

        // resolve over the decay curve
        let input = ResolvedInput {
            token: self.baseInput.token.to_string(),
            amount: match self.baseInput.curve.decay(
                self.baseInput.startAmount,
                self.cosignerData.decayStartBlock,
                BigUint::from(block_number),
                BigUint::from(0),
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
                    BigUint::from(block_number),
                    output.minAmount,
                    BigUint::MAX,
                    NonlinearDutchDecay::v3_linear_output_decay
                )?;
                
                // add exclusivity override to amount if before decay start block
                if self.cosignerData.decayStartBlock.gt(&BigUint::from(block_number)) && !self.cosignerData.exclusiveFiller.is_zero() {
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

fn resolve_decay(
    at_time: BigUint,
    start_time: BigUint,
    end_time: BigUint,
    start_amount: BigUint,
    end_amount: BigUint,
) -> BigUint {
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

    let duration = end_time.wrapping_sub(start_time);
    let elapsed = at_time.wrapping_sub(start_time);
    // TODO: better handle overflows
    if start_amount.gt(&end_amount) {
        // decaying downward
        let decay = start_amount
            .wrapping_sub(end_amount)
            .wrapping_mul(elapsed)
            .wrapping_div(duration);
        return start_amount.wrapping_sub(decay);
    } else {
        // decaying upward
        let decay = end_amount
            .wrapping_sub(start_amount)
            .wrapping_mul(elapsed)
            .wrapping_div(duration);
        return start_amount.wrapping_add(decay);
    }
}

impl NonlinearDutchDecay {

    pub fn decay(
        &self,
        start_amount: BigUint,
        decay_start_block: BigUint,
        block_numberish: BigUint,
        min_amount: BigUint,
        max_amount: BigUint,
        decay_func: fn(BigUint, BigUint, BigUint, I256, I256) -> Result<I256>
    ) -> Result<BigUint> {
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
            (block_numberish - decay_start_block).min(BigUint::from(u16::MAX))
        )?;

        let (start_point, end_point, rel_start_amount, rel_end_amount) = 
            self.locate_curve_position(block_delta)?;

        // Calculate decay of only the relative amounts
        let curve_delta = (decay_func)(
            BigUint::from(start_point),
            BigUint::from(end_point),
            BigUint::from(block_delta),
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
        start_point: BigUint,
        end_point: BigUint,
        current_point: BigUint,
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
                BigUint::try_from(start_amount.checked_sub(end_amount)
                    .ok_or_else(|| anyhow::anyhow!("Underflow in start_amount - end_amount"))?)?
                    .mul_div_down(elapsed, duration)
                    .map_err(|e| anyhow::anyhow!("MulDivDown error: {}", e))?
            )?);
        } else {
            delta = I256::try_from(
                BigUint::try_from(end_amount.checked_sub(start_amount)
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
        start_point: BigUint,
        end_point: BigUint,
        current_point: BigUint,
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
                BigUint::try_from(start_amount.checked_sub(end_amount)
                    .ok_or_else(|| anyhow::anyhow!("Underflow in start_amount - end_amount"))?)?
                    .mul_div_up(elapsed, duration)
                    .map_err(|e| anyhow::anyhow!("MulDivUp error: {}", e))?
            )?);
        } else {
            delta = I256::try_from(
                BigUint::try_from(end_amount.checked_sub(start_amount)
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
    pub fn to_uint16_array(input_array: &[u16]) -> Result<BigUint> {
        if input_array.len() > PACKED_UINT16_ARRAY_LENGTH {
            return Err(anyhow::Error::msg("Invalid array length"));
        }

        let mut packed_data = BigUint::ZERO;

        for (i, &value) in input_array.iter().enumerate() {
            let shifted_value = BigUint::from(value as u64) << (i * 16);
            packed_data |= shifted_value;
        }

        Ok(packed_data)
    }

    
    /// Retrieve the nth uint16 value from a packed uint256
    fn get_element(packed_data: BigUint, n: usize) -> Result<u16> {
        if n >= PACKED_UINT16_ARRAY_LENGTH {
            return Err(anyhow::Error::msg("IndexOutOfBounds"));
        }
        
        let shift_amount = n * 16;
        let masked_value = (packed_data >> shift_amount) & BigUint::from(0xFFFF);
        let result = u16::try_from(masked_value)?;
        Ok(result)
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    const DECAY_FUNCTIONS: [fn(BigUint, BigUint, BigUint, I256, I256) -> Result<I256>; 2] = [
        NonlinearDutchDecay::v3_linear_input_decay,
        NonlinearDutchDecay::v3_linear_output_decay
    ];

    #[test]
    fn test_decay_after_end_time() {
        let start_time = BigUint::from(1);
        let end_time = BigUint::from(10);
        let start_amount = BigUint::from(100000);
        let end_amount = BigUint::from(100000000);

        let at_time = BigUint::from(11);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, end_amount);
    }

    #[test]
    fn test_decay_at_end_time() {
        let start_time = BigUint::from(1);
        let end_time = BigUint::from(10);
        let start_amount = BigUint::from(100000);
        let end_amount = BigUint::from(100000000);

        let at_time = BigUint::from(10);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, end_amount);
    }

    #[test]
    fn test_decay_before_start_time() {
        let start_time = BigUint::from(10);
        let end_time = BigUint::from(100);
        let start_amount = BigUint::from(100000);
        let end_amount = BigUint::from(100000000);

        let at_time = BigUint::from(5);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, start_amount);
    }

    #[test]
    fn test_decay_at_start_time() {
        let start_time = BigUint::from(10);
        let end_time = BigUint::from(100);
        let start_amount = BigUint::from(100000);
        let end_amount = BigUint::from(100000000);

        let at_time = BigUint::from(10);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, start_amount);
    }

    #[test]
    fn test_upwards_decay() {
        let start_time = BigUint::from(10);
        let end_time = BigUint::from(20);
        let start_amount = BigUint::from(100000);
        let end_amount = BigUint::from(200000);

        let at_time = BigUint::from(15);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, BigUint::from(150000));
    }

    #[test]
    fn test_downwards_decay() {
        let start_time = BigUint::from(10);
        let end_time = BigUint::from(20);
        let start_amount = BigUint::from(200000);
        let end_amount = BigUint::from(100000);

        let at_time = BigUint::from(15);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, BigUint::from(150000));
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

        let start_block = BigUint::from(1000);
        let current_block = BigUint::from(999);
        let start_amount = BigUint::from(1000);
        let min_amount = BigUint::from(0);
        let max_amount = BigUint::MAX;

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

        let start_block = BigUint::from(1000);
        let current_block = BigUint::from(1000);
        let start_amount = BigUint::from(1000);
        let min_amount = BigUint::from(0);
        let max_amount = BigUint::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );

            assert_eq!(result.unwrap(), BigUint::from(1000));
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

        let start_block = BigUint::from(1000);
        let current_block = BigUint::from(1150);
        let start_amount = BigUint::from(1000);
        let min_amount = BigUint::from(0);
        let max_amount = BigUint::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );

            assert_eq!(result.unwrap(), BigUint::from(100));
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

        let start_block = BigUint::from(1000);
        let current_block = BigUint::from(1500);
        let start_amount = BigUint::from(1000);
        let min_amount = BigUint::from(0);
        let max_amount = BigUint::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );

            assert_eq!(result.unwrap(), BigUint::from(800));
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

        let start_block = BigUint::from(1000);
        let current_block = BigUint::from(1600);
        let start_amount = BigUint::from(1000);
        let min_amount = BigUint::from(0);
        let max_amount = BigUint::MAX;

        for decay_func in DECAY_FUNCTIONS.iter() {
            let result = decay.decay(
                start_amount,
                start_block,
                current_block,
                min_amount,
                max_amount,
                *decay_func
            );
        assert_eq!(result.unwrap(), BigUint::from(800));
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

        let start_block = BigUint::from(1000);
        let current_block = BigUint::from(1100);
        let start_amount = BigUint::from(1000);
        let min_amount = BigUint::from(300);
        let max_amount = BigUint::MAX;

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

        let start_block = BigUint::from(1000);
        let current_block = BigUint::from(1500);
        let start_amount = BigUint::from(1000);
        let min_amount = BigUint::from(0);
        let max_amount = BigUint::from(500);

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

        let start_block = BigUint::from(1000);
        let current_block = BigUint::from(1100);
        let start_amount = BigUint::from(500); // Less than max relativeAmount
        let min_amount = BigUint::from(10);
        let max_amount = BigUint::MAX;

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

        let start_block = BigUint::from(1000);
        let current_block = BigUint::from(1100);
        let start_amount = BigUint::MAX;
        let min_amount = BigUint::from(10);
        let max_amount = BigUint::MAX;

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
        let start_block = BigUint::from(1000);
        let current_block = BigUint::from(1100);
        let start_amount = BigUint::from(1000);
        let min_amount = BigUint::from(0);
        let max_amount = BigUint::MAX;

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
