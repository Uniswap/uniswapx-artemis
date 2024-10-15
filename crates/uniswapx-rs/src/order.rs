use std::error::Error;

use alloy_dyn_abi::SolType;
use alloy_primitives::Uint;
use alloy_primitives::I256;
use alloy_sol_types::sol;
use anyhow::Result;

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
pub const BPS: Uint<256, 4> = Uint::from_limbs([10000, 0, 0, 0]);

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
    pub amount: Uint<256, 4>,
}

#[derive(Debug, Clone)]
pub struct ResolvedOutput {
    pub token: String,
    pub amount: Uint<256, 4>,
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
        Ok(V2DutchOrder::decode_single(order_hex, validate)?)
    }

    pub fn encode_inner(&self) -> Vec<u8> {
        V2DutchOrder::encode_single(self)
    }

    pub fn resolve(&self, timestamp: u64) -> OrderResolution {
        let timestamp = Uint::from(timestamp);

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

        let outputs = self
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
                    let exclusivity = self.cosignerData.exclusivityOverrideBps.wrapping_add(BPS);
                    let exclusivity = exclusivity.wrapping_mul(amount);
                    amount = exclusivity.wrapping_div(BPS);
                };

                ResolvedOutput {
                    token: output.token.to_string(),
                    amount,
                    recipient: output.recipient.to_string(),
                }
            })
            .collect();

        OrderResolution::Resolved(ResolvedOrder { input, outputs })
    }
}

impl PriorityOrder {
    pub fn decode_inner(order_hex: &[u8], validate: bool) -> Result<Self, Box<dyn Error>> {
        Ok(PriorityOrder::decode_single(order_hex, validate)?)
    }

    pub fn encode_inner(&self) -> Vec<u8> {
        PriorityOrder::encode_single(self)
    }

    pub fn resolve(&self, block_number: u64, timestamp: u64, priority_fee: Uint<256, 4>) -> OrderResolution {
        let timestamp = Uint::from(timestamp);

        if self.info.deadline.lt(&timestamp) {
            return OrderResolution::Expired;
        };

        let input = self.input.scale(priority_fee);
        let outputs = self
            .outputs
            .iter()
            .map(|output| output.scale(priority_fee))
            .collect();

        if Uint::from(block_number).lt(&self.cosignerData.auctionTargetBlock.saturating_sub(Uint::from(2))) {
            return OrderResolution::NotFillableYet(ResolvedOrder { input, outputs });
        };

        OrderResolution::Resolved(ResolvedOrder { input, outputs })
    }
}

impl PriorityInput {
    pub fn scale(&self, priority_fee: Uint<256, 4>) -> ResolvedInput {
        let amount = self.amount.wrapping_mul(Uint::from(MPS).wrapping_add(priority_fee.wrapping_mul(self.mpsPerPriorityFeeWei))).wrapping_div(Uint::from(MPS));
        ResolvedInput {
            token: self.token.to_string(),
            amount,
        }
    }
}

impl PriorityOutput {
    pub fn scale(&self, priority_fee: Uint<256, 4>) -> ResolvedOutput {
        let amount = self.amount.wrapping_mul(Uint::from(MPS).saturating_sub(priority_fee.wrapping_mul(self.mpsPerPriorityFeeWei))).wrapping_div(Uint::from(MPS));
        ResolvedOutput {
            token: self.token.to_string(),
            amount,
            recipient: self.recipient.to_string(),
        }
    }
}

impl V3DutchOrder {
    pub fn decode_inner(order_hex: &[u8], validate: bool) -> Result<Self, Box<dyn Error>> {
        Ok(V3DutchOrder::decode_single(order_hex, validate)?)
    }

    pub fn encode_inner(&self) -> Vec<u8> {
        V3DutchOrder::encode_single(self)
    }

    pub fn resolve(&self, block_number: u64, timestamp: u64) -> OrderResolution {
        let timestamp = Uint::from(timestamp);

        if self.info.deadline.lt(&timestamp) {
            return OrderResolution::Expired;
        };

        // resolve over the decay curve
        // TODO: apply cosigner logic

        let input = ResolvedInput {
            token: self.baseInput.token.to_string(),
            amount: match self.baseInput.curve.decay(
                self.baseInput.startAmount,
                self.cosignerData.decayStartBlock,
                Uint::from(block_number),
                Uint::from(0),
                self.baseInput.maxAmount,
            ) {
                Ok(amount) => amount,
                Err(_) => return OrderResolution::Invalid,
            },
        };

        let outputs: Result<Vec<ResolvedOutput>, Box<dyn Error>> = self
            .baseOutputs
            .iter()
            .map(|output| {
                let mut amount = output.curve.decay(
                    output.startAmount,
                    self.cosignerData.decayStartBlock,
                    Uint::from(block_number),
                    output.minAmount,
                    Uint::MAX,
                )?;
                
                // add exclusivity override to amount if before decay start block
                if self.cosignerData.decayStartBlock.gt(&Uint::from(block_number)) && !self.cosignerData.exclusiveFiller.is_zero() {
                    let exclusivity = self.cosignerData.exclusivityOverrideBps.wrapping_add(BPS);
                    let exclusivity = exclusivity.wrapping_mul(amount);
                    amount = exclusivity.wrapping_div(BPS);
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
    at_time: Uint<256, 4>,
    start_time: Uint<256, 4>,
    end_time: Uint<256, 4>,
    start_amount: Uint<256, 4>,
    end_amount: Uint<256, 4>,
) -> Uint<256, 4> {
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

    /// Calculates the decayed amount based on the current block and the defined curve
    pub fn decay(
        &self,
        start_amount: Uint<256, 4>,
        decay_start_block: Uint<256, 4>,
        current_block: Uint<256, 4>,
        min_amount: Uint<256, 4>,
        max_amount: Uint<256, 4>,
    ) -> Result<Uint<256, 4>, Box<dyn Error>> {
        // Check for invalid decay curve
        if self.relativeAmounts.len() > 16 {
            return Err("Invalid decay curve".into());
        }

        // Handle current block before decay or no decay
        if decay_start_block >= current_block || self.relativeAmounts.is_empty() {
            return Ok(start_amount.clamp(min_amount, max_amount));
        }

        // Cap block_delta to u16::MAX to prevent overflow
        let block_delta: u16 = u16::try_from(
            (current_block - decay_start_block).min(Uint::<256, 4>::from(u16::MAX))
        )?;
        println!("block_delta: {}", block_delta);

        let (start_point, end_point, rel_start_amount, rel_end_amount) = 
            self.locate_curve_position(block_delta)?;
        println!("start_point: {}", start_point);
        println!("end_point: {}", end_point);
        println!("rel_start_amount: {}", rel_start_amount);
        println!("rel_end_amount: {}", rel_end_amount);

        // Calculate decay of only the relative amounts
        let curve_delta = resolve_decay(
            Uint::<256, 4>::from(block_delta),
            Uint::<256, 4>::from(start_point),
            Uint::<256, 4>::from(end_point),
            Uint::<256, 4>::try_from(rel_start_amount).unwrap(),
            Uint::<256, 4>::try_from(rel_end_amount).unwrap(),
        );

        // Apply curve_delta to start_amount and bound the result
        start_amount
            .checked_sub(Uint::<256, 4>::try_from(curve_delta)?)
            .ok_or("Underflow".into())
            .map(|result| result.clamp(min_amount, max_amount))
    }

    /// Locates the position on the decay curve based on the current block
    fn locate_curve_position(&self, current_relative_block: u16) -> Result<(u16, u16, I256, I256), Box<dyn Error>> {
        println!("comparing {} with {}", Self::get_element(self.relativeBlocks, 0)?, current_relative_block);
        // Position is before the start of the curve
        if Self::get_element(self.relativeBlocks, 0)? >= current_relative_block {
            return Ok((0, Self::get_element(self.relativeBlocks, 0)?, I256::ZERO, self.relativeAmounts[0]));
        }
        println!("position in curve");
        let last_curve_index = self.relativeAmounts.len() - 1;
        for i in 1..=last_curve_index {
            println!("comparing {} with {}", Self::get_element(self.relativeBlocks, i)?, current_relative_block);
            if Self::get_element(self.relativeBlocks, i)? >= current_relative_block {
                println!("found it!");
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
    /// * `Result<Uint<256, 4>, Box<dyn std::error::Error>>` - The packed Uint value or an error
    pub fn to_uint16_array(input_array: &[u16]) -> std::result::Result<Uint<256, 4>, Box<dyn std::error::Error>> {
        if input_array.len() > 16 {
            return Err("Invalid array length".into());
        }

        let mut packed_data = Uint::<256, 4>::ZERO;

        for (i, &value) in input_array.iter().enumerate() {
            let shifted_value = Uint::<256, 4>::from(value as u64) << (i * 16);
            packed_data |= shifted_value;
        }

        Ok(packed_data)
    }

    
    /// Retrieve the nth uint16 value from a packed uint256
    fn get_element(packed_data: Uint<256, 4>, n: usize) -> Result<u16, Box<dyn std::error::Error>> {
        if n >= 16 {
            return Err("IndexOutOfBounds".into());
        }
        
        let shift_amount = n * 16;
        let masked_value = (packed_data >> shift_amount) & Uint::<256, 4>::from(0xFFFF);
        let result = u16::try_from(masked_value)?;
        Ok(result)
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay_after_end_time() {
        let start_time = Uint::from(1);
        let end_time = Uint::from(10);
        let start_amount = Uint::from(100000);
        let end_amount = Uint::from(100000000);

        let at_time = Uint::from(11);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, end_amount);
    }

    #[test]
    fn test_decay_at_end_time() {
        let start_time = Uint::from(1);
        let end_time = Uint::from(10);
        let start_amount = Uint::from(100000);
        let end_amount = Uint::from(100000000);

        let at_time = Uint::from(10);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, end_amount);
    }

    #[test]
    fn test_decay_before_start_time() {
        let start_time = Uint::from(10);
        let end_time = Uint::from(100);
        let start_amount = Uint::from(100000);
        let end_amount = Uint::from(100000000);

        let at_time = Uint::from(5);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, start_amount);
    }

    #[test]
    fn test_decay_at_start_time() {
        let start_time = Uint::from(10);
        let end_time = Uint::from(100);
        let start_amount = Uint::from(100000);
        let end_amount = Uint::from(100000000);

        let at_time = Uint::from(10);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, start_amount);
    }

    #[test]
    fn test_upwards_decay() {
        let start_time = Uint::from(10);
        let end_time = Uint::from(20);
        let start_amount = Uint::from(100000);
        let end_amount = Uint::from(200000);

        let at_time = Uint::from(15);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, Uint::from(150000));
    }

    #[test]
    fn test_downwards_decay() {
        let start_time = Uint::from(10);
        let end_time = Uint::from(20);
        let start_amount = Uint::from(200000);
        let end_amount = Uint::from(100000);

        let at_time = Uint::from(15);

        let result = resolve_decay(at_time, start_time, end_time, start_amount, end_amount);

        assert_eq!(result, Uint::from(150000));
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

        let start_block = Uint::from(1000);
        let current_block = Uint::from(999);
        let start_amount = Uint::from(1000);
        let min_amount = Uint::from(0);
        let max_amount = Uint::MAX;

        let result = decay.decay(start_amount, start_block, current_block, min_amount, max_amount);

        assert_eq!(result.unwrap(), start_amount);
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

        let start_block = Uint::from(1000);
        let current_block = Uint::from(1000);
        let start_amount = Uint::from(1000);
        let min_amount = Uint::from(0);
        let max_amount = Uint::MAX;

        let result = decay.decay(start_amount, start_block, current_block, min_amount, max_amount);

        assert_eq!(result.unwrap(), Uint::from(1000));
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

        let start_block = Uint::from(1000);
        let current_block = Uint::from(1150);
        let start_amount = Uint::from(1000);
        let min_amount = Uint::from(0);
        let max_amount = Uint::MAX;

        let result = decay.decay(start_amount, start_block, current_block, min_amount, max_amount);

        assert_eq!(result.unwrap(), Uint::from(100));
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

        let start_block = Uint::from(1000);
        let current_block = Uint::from(1500);
        let start_amount = Uint::from(1000);
        let min_amount = Uint::from(0);
        let max_amount = Uint::MAX;

        let result = decay.decay(start_amount, start_block, current_block, min_amount, max_amount);

        assert_eq!(result.unwrap(), Uint::from(800));
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

        let start_block = Uint::from(1000);
        let current_block = Uint::from(1600);
        let start_amount = Uint::from(1000);
        let min_amount = Uint::from(0);
        let max_amount = Uint::MAX;

        let result = decay.decay(start_amount, start_block, current_block, min_amount, max_amount);

        assert_eq!(result.unwrap(), Uint::from(800));
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

        let start_block = Uint::from(1000);
        let current_block = Uint::from(1100);
        let start_amount = Uint::from(1000);
        let min_amount = Uint::from(300);
        let max_amount = Uint::MAX;

        let result = decay.decay(start_amount, start_block, current_block, min_amount, max_amount);

        assert_eq!(result.unwrap(), Uint::from(300));
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

        let start_block = Uint::from(1000);
        let current_block = Uint::from(1500);
        let start_amount = Uint::from(1000);
        let min_amount = Uint::from(0);
        let max_amount = Uint::from(500);

        let result = decay.decay(start_amount, start_block, current_block, min_amount, max_amount);

        assert_eq!(result.unwrap(), Uint::from(500));
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

        let start_block = Uint::from(1000);
        let current_block = Uint::from(1100);
        let start_amount = Uint::from(500); // Less than max relativeAmount
        let min_amount = Uint::from(0);
        let max_amount = Uint::MAX;

        let result = decay.decay(start_amount, start_block, current_block, min_amount, max_amount);
        assert!(result.is_err());
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
        let start_block = Uint::from(1000);
        let current_block = Uint::from(1100);
        let start_amount = Uint::from(1000);
        let min_amount = Uint::from(0);
        let max_amount = Uint::MAX;

        let result = decay_empty.decay(start_amount, start_block, current_block, min_amount, max_amount);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), start_amount);
    }
}