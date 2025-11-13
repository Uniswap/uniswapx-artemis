use anyhow::Result;
use ethabi::{ethereum_types::U256, Token};
use std::convert::TryInto;

const FUNCTION_SELECTOR_BYTE_LENGTH: usize = 4;

// Function selectors
pub const UNIVERSAL_ROUTER_EXECUTE_SELECTOR: [u8; 4] = [0x35, 0x93, 0x56, 0x4c]; // keccak256("execute(bytes,bytes[],uint256)")
pub const EXACT_INPUT_SELECTOR: [u8; 4] = [0xb8, 0x58, 0x18, 0x3f]; // keccak256("exactInput(bytes,address,uint256,uint256)")
pub const EXACT_INPUT_SINGLE_SELECTOR: [u8; 4] = [0x04, 0xe4, 0x5a, 0xaf]; // keccak256("exactInputSingle(address,address,uint24,address,uint256,uint256,uint160)")
pub const EXACT_OUTPUT_SELECTOR: [u8; 4] = [0x09, 0x0b, 0x5c, 0xb5]; // keccak256("exactOutput(bytes,address,uint256,uint256)")
pub const EXACT_OUTPUT_SINGLE_SELECTOR: [u8; 4] = [0x50, 0x0b, 0x5d, 0x1d]; // keccak256("exactOutputSingle(address,address,uint24,address,uint256,uint256,uint160)")

// Universal Router commands
pub const V3_SWAP_EXACT_IN: u8 = 0x00;
pub const V3_SWAP_EXACT_OUT: u8 = 0x01;
pub const PERMIT2_PERMIT: u8 = 0x0a;
pub const WRAP_ETH: u8 = 0x0b;
pub const UNWRAP_WETH: u8 = 0x0c;
pub const PERMIT2_TRANSFER_FROM: u8 = 0x0d;
pub const SWEEP: u8 = 0x0e;
pub const TRANSFER: u8 = 0x0f;
pub const PAY_PORTION: u8 = 0x10;
pub const V2_SWAP_EXACT_IN: u8 = 0x11;
pub const V2_SWAP_EXACT_OUT: u8 = 0x12;
pub const PERMIT: u8 = 0x13;
pub const WRAP_ETH2: u8 = 0x14;
pub const UNWRAP_WETH2: u8 = 0x15;
pub const BALANCE_CHECK: u8 = 0x16;
pub const V4_SWAP: u8 = 0x10;

// ABI parameter helper functions
pub fn universal_router_execute_params() -> Vec<ethabi::ParamType> {
    vec![
        ethabi::ParamType::Bytes,
        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes)),
        ethabi::ParamType::Uint(256),
    ]
}

pub fn exact_input_tuple_params() -> ethabi::ParamType {
    ethabi::ParamType::Tuple(vec![
        ethabi::ParamType::Bytes,
        ethabi::ParamType::Address,
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
    ])
}

pub fn exact_input_single_tuple_params() -> ethabi::ParamType {
    ethabi::ParamType::Tuple(vec![
        ethabi::ParamType::Address,
        ethabi::ParamType::Address,
        ethabi::ParamType::Uint(24),
        ethabi::ParamType::Address,
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(256),
        ethabi::ParamType::Uint(160),
    ])
}

pub fn exact_output_tuple_params() -> ethabi::ParamType {
    ethabi::ParamType::Tuple(vec![
        ethabi::ParamType::Bytes,
        ethabi::ParamType::Address,
        ethabi::ParamType::Uint(256), // amountOut
        ethabi::ParamType::Uint(256), // amountInMaximum
    ])
}

pub fn exact_output_single_tuple_params() -> ethabi::ParamType {
    ethabi::ParamType::Tuple(vec![
        ethabi::ParamType::Address,
        ethabi::ParamType::Address,
        ethabi::ParamType::Uint(24),
        ethabi::ParamType::Address,
        ethabi::ParamType::Uint(256), // amountOut
        ethabi::ParamType::Uint(256), // amountInMaximum
        ethabi::ParamType::Uint(160),
    ])
}

/// Represents a parsed Universal Router command
#[derive(Debug, Clone)]
pub struct UniversalRouterCommand {
    pub command: u8,
    pub data: Vec<u8>,
}

/// Represents a parsed Universal Router execute call
#[derive(Debug, Clone)]
pub struct UniversalRouterExecute {
    pub commands: Vec<UniversalRouterCommand>,
    pub swap_calls: Vec<Vec<u8>>,
    pub deadline: U256,
}

/// Parser for Universal Router calldata
pub struct UniversalRouterParser;

impl UniversalRouterParser {
    /// Parse Universal Router execute calldata
    pub fn parse_execute(calldata: &[u8]) -> Result<UniversalRouterExecute> {
        if calldata.len() < FUNCTION_SELECTOR_BYTE_LENGTH {
            anyhow::bail!("Calldata too short");
        }

        let (selector, params) = calldata.split_at(FUNCTION_SELECTOR_BYTE_LENGTH);
        let selector: [u8; FUNCTION_SELECTOR_BYTE_LENGTH] = selector.try_into()?;

        if selector != UNIVERSAL_ROUTER_EXECUTE_SELECTOR {
            anyhow::bail!("Not Universal Router execute calldata");
        }

        let decoded_params = ethabi::decode(&universal_router_execute_params(), params)?;

        if let [Token::Bytes(commands), Token::Array(swap_calls), Token::Uint(deadline)] = decoded_params.as_slice() {
            let parsed_commands = Self::parse_commands(commands)?;
            
            let swap_calls_bytes: Vec<Vec<u8>> = swap_calls
                .iter()
                .filter_map(|call| {
                    if let Token::Bytes(bytes) = call {
                        Some(bytes.clone())
                    } else {
                        None
                    }
                })
                .collect();

            Ok(UniversalRouterExecute {
                commands: parsed_commands,
                swap_calls: swap_calls_bytes,
                deadline: deadline.clone(),
            })
        } else {
            anyhow::bail!("Invalid Universal Router execute params format");
        }
    }

    /// Parse commands from the commands bytes
    pub fn parse_commands(commands: &[u8]) -> Result<Vec<UniversalRouterCommand>> {
        let mut parsed_commands = Vec::new();
        let mut offset = 0;

        while offset < commands.len() {
            if offset + 1 > commands.len() {
                anyhow::bail!("Incomplete command at offset {}", offset);
            }

            let command = commands[offset];
            offset += 1;

            // For now, we'll handle the basic structure
            // Each command can have different data lengths
            let data_length = match command {
                V3_SWAP_EXACT_IN => 7 * 32, // 7 parameters * 32 bytes each
                V3_SWAP_EXACT_OUT => 7 * 32,
                V2_SWAP_EXACT_IN => 5 * 32,
                V2_SWAP_EXACT_OUT => 5 * 32,
                V4_SWAP => {
                    // V4_SWAP uses a custom V4Planner encoding
                    // The data length is variable and encoded in the first 32 bytes
                    if offset + 32 > commands.len() {
                        anyhow::bail!("Incomplete V4_SWAP command data at offset {}", offset);
                    }
                    
                    // Read the length from the first 32 bytes
                    let length_bytes = &commands[offset..offset + 32];
                    let length = ethabi::ethereum_types::U256::from_big_endian(length_bytes);
                    let length_usize = length.as_usize();
                    
                    if length_usize == 0 {
                        anyhow::bail!("Invalid V4_SWAP command: zero length data");
                    }
                    
                    if offset + 32 + length_usize > commands.len() {
                        anyhow::bail!("Incomplete V4_SWAP command data: expected {} bytes but only {} available", 
                                     length_usize, commands.len() - offset - 32);
                    }
                    
                    32 + length_usize // 32 bytes for length + actual data length
                }
                _ => {
                    // For unknown commands, try to read at least 32 bytes or until end
                    let remaining = commands.len() - offset;
                    if remaining >= 32 {
                        32
                    } else {
                        remaining
                    }
                }
            };

            if offset + data_length > commands.len() {
                anyhow::bail!("Incomplete command data at offset {}", offset);
            }

            let data = commands[offset..offset + data_length].to_vec();
            parsed_commands.push(UniversalRouterCommand { command, data });
            offset += data_length;
        }

        Ok(parsed_commands)
    }

    /// Extract amountOutMinimum from V3_SWAP_EXACT_IN command
    pub fn extract_v3_swap_exact_in_amount(commands: &[u8]) -> Result<U256> {
        let parsed_commands = Self::parse_commands(commands)?;
        
        for cmd in parsed_commands {
            if cmd.command == V3_SWAP_EXACT_IN {
                if cmd.data.len() >= 7 * 32 {
                    // amountOutMinimum is the 6th parameter (0-indexed)
                    let amount_out_min_offset = 5 * 32;
                    let amount_out_min_bytes = &cmd.data[amount_out_min_offset..amount_out_min_offset + 32];
                    return Ok(U256::from_big_endian(amount_out_min_bytes));
                }
            }
        }
        
        anyhow::bail!("V3_SWAP_EXACT_IN command not found or invalid data length");
    }

    /// Extract amountOutMinimum from V3_SWAP_EXACT_OUT command
    pub fn extract_v3_swap_exact_out_amount(commands: &[u8]) -> Result<U256> {
        let parsed_commands = Self::parse_commands(commands)?;
        
        for cmd in parsed_commands {
            if cmd.command == V3_SWAP_EXACT_OUT {
                if cmd.data.len() >= 7 * 32 {
                    // amountInMaximum is the 6th parameter (0-indexed)
                    let amount_in_max_offset = 5 * 32;
                    let amount_in_max_bytes = &cmd.data[amount_in_max_offset..amount_in_max_offset + 32];
                    return Ok(U256::from_big_endian(amount_in_max_bytes));
                }
            }
        }
        
        anyhow::bail!("V3_SWAP_EXACT_OUT command not found or invalid data length");
    }

    /// Extract amountOutMinimum from V4_SWAP command
    /// Note: V4 swaps use a complex V4Planner encoding that requires more detailed parsing
    pub fn extract_v4_swap_amount(commands: &[u8]) -> Result<U256> {
        let parsed_commands = Self::parse_commands(commands)?;
        
        for cmd in parsed_commands {
            if cmd.command == V4_SWAP {
                // V4_SWAP uses V4Planner encoding which is more complex
                // For now, return an error indicating this needs more implementation
                anyhow::bail!("V4_SWAP amount extraction not yet implemented - requires V4Planner decoding");
            }
        }
        
        anyhow::bail!("V4_SWAP command not found");
    }

    /// Create new V3_SWAP_EXACT_IN command with updated amountOutMinimum
    pub fn create_v3_swap_exact_in_with_amount(commands: &[u8], new_amount: U256) -> Result<Vec<u8>> {
        let parsed_commands = Self::parse_commands(commands)?;
        let mut new_commands = Vec::new();

        for cmd in parsed_commands {
            if cmd.command == V3_SWAP_EXACT_IN {
                if cmd.data.len() >= 7 * 32 {
                    let mut new_data = cmd.data.clone();
                    let amount_out_min_offset = 5 * 32;
                    let mut new_amount_bytes = [0u8; 32];
                    new_amount.to_big_endian(&mut new_amount_bytes);
                    new_data[amount_out_min_offset..amount_out_min_offset + 32].copy_from_slice(&new_amount_bytes);
                    new_commands.push(UniversalRouterCommand { command: cmd.command, data: new_data });
                } else {
                    new_commands.push(cmd);
                }
            } else {
                new_commands.push(cmd);
            }
        }

        Self::encode_commands(&new_commands)
    }

    /// Create new V3_SWAP_EXACT_OUT command with updated amountInMaximum
    pub fn create_v3_swap_exact_out_with_amount(commands: &[u8], new_amount: U256) -> Result<Vec<u8>> {
        let parsed_commands = Self::parse_commands(commands)?;
        let mut new_commands = Vec::new();

        for cmd in parsed_commands {
            if cmd.command == V3_SWAP_EXACT_OUT {
                if cmd.data.len() >= 7 * 32 {
                    let mut new_data = cmd.data.clone();
                    let amount_in_max_offset = 5 * 32;
                    let mut new_amount_bytes = [0u8; 32];
                    new_amount.to_big_endian(&mut new_amount_bytes);
                    new_data[amount_in_max_offset..amount_in_max_offset + 32].copy_from_slice(&new_amount_bytes);
                    new_commands.push(UniversalRouterCommand { command: cmd.command, data: new_data });
                } else {
                    new_commands.push(cmd);
                }
            } else {
                new_commands.push(cmd);
            }
        }

        Self::encode_commands(&new_commands)
    }

    /// Create new V4_SWAP command with updated amount
    /// Note: V4 swaps use a complex V4Planner encoding that requires more detailed implementation
    pub fn create_v4_swap_with_amount(commands: &[u8], _new_amount: U256) -> Result<Vec<u8>> {
        let parsed_commands = Self::parse_commands(commands)?;
        let mut new_commands = Vec::new();

        for cmd in parsed_commands {
            if cmd.command == V4_SWAP {
                // V4_SWAP uses V4Planner encoding which is more complex
                // For now, return an error indicating this needs more implementation
                anyhow::bail!("V4_SWAP amount modification not yet implemented - requires V4Planner encoding");
            } else {
                new_commands.push(cmd);
            }
        }

        Self::encode_commands(&new_commands)
    }

    /// Encode commands back to bytes
    pub fn encode_commands(commands: &[UniversalRouterCommand]) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        
        for cmd in commands {
            encoded.push(cmd.command);
            encoded.extend_from_slice(&cmd.data);
        }
        
        Ok(encoded)
    }

    /// Check if calldata is a Universal Router execute call
    pub fn is_universal_router_execute(calldata: &[u8]) -> bool {
        if calldata.len() < FUNCTION_SELECTOR_BYTE_LENGTH {
            return false;
        }
        
        let (selector, _) = calldata.split_at(FUNCTION_SELECTOR_BYTE_LENGTH);
        let selector: [u8; FUNCTION_SELECTOR_BYTE_LENGTH] = match selector.try_into() {
            Ok(sel) => sel,
            Err(_) => return false,
        };
        
        selector == UNIVERSAL_ROUTER_EXECUTE_SELECTOR
    }

    /// Check if a swap call is a known swap function
    pub fn is_known_swap_function(swap_call: &[u8]) -> bool {
        if swap_call.len() < FUNCTION_SELECTOR_BYTE_LENGTH {
            return false;
        }
        
        let (selector, _) = swap_call.split_at(FUNCTION_SELECTOR_BYTE_LENGTH);
        let selector: [u8; FUNCTION_SELECTOR_BYTE_LENGTH] = match selector.try_into() {
            Ok(sel) => sel,
            Err(_) => return false,
        };
        
        matches!(selector, 
            EXACT_INPUT_SELECTOR | 
            EXACT_INPUT_SINGLE_SELECTOR | 
            EXACT_OUTPUT_SELECTOR | 
            EXACT_OUTPUT_SINGLE_SELECTOR
        )
    }

    /// Check if a swap call uses command-based encoding (selector is [0, 0, 0, 0])
    pub fn is_command_based_encoding(swap_call: &[u8]) -> bool {
        if swap_call.len() < FUNCTION_SELECTOR_BYTE_LENGTH {
            return false;
        }
        
        let (selector, _) = swap_call.split_at(FUNCTION_SELECTOR_BYTE_LENGTH);
        let selector: [u8; FUNCTION_SELECTOR_BYTE_LENGTH] = match selector.try_into() {
            Ok(sel) => sel,
            Err(_) => return false,
        };
        
        selector == [0, 0, 0, 0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethabi::encode;

    #[test]
    fn test_parse_commands() {
        // Create a simple V3_SWAP_EXACT_IN command
        let mut commands = vec![V3_SWAP_EXACT_IN];
        let data = vec![0u8; 7 * 32]; // 7 parameters * 32 bytes each
        commands.extend_from_slice(&data);
        
        let parsed = UniversalRouterParser::parse_commands(&commands).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].command, V3_SWAP_EXACT_IN);
        assert_eq!(parsed[0].data.len(), 7 * 32);
    }

    #[test]
    fn test_parse_commands_multiple() {
        // Create multiple commands
        let mut commands = vec![V3_SWAP_EXACT_IN];
        let data1 = vec![0u8; 7 * 32];
        commands.extend_from_slice(&data1);
        
        commands.push(V3_SWAP_EXACT_OUT);
        let data2 = vec![0u8; 7 * 32];
        commands.extend_from_slice(&data2);
        
        let parsed = UniversalRouterParser::parse_commands(&commands).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].command, V3_SWAP_EXACT_IN);
        assert_eq!(parsed[1].command, V3_SWAP_EXACT_OUT);
    }

    #[test]
    fn test_extract_v3_swap_exact_in_amount() {
        let mut commands = vec![V3_SWAP_EXACT_IN];
        let mut data = vec![0u8; 7 * 32];
        
        // Set amountOutMinimum to a specific value (6th parameter)
        let amount = U256::from(1000000);
        let mut amount_bytes = [0u8; 32];
        amount.to_big_endian(&mut amount_bytes);
        data[5 * 32..6 * 32].copy_from_slice(&amount_bytes);
        
        commands.extend_from_slice(&data);
        
        let extracted = UniversalRouterParser::extract_v3_swap_exact_in_amount(&commands).unwrap();
        assert_eq!(extracted, amount);
    }

    #[test]
    fn test_create_v3_swap_exact_in_with_amount() {
        let mut commands = vec![V3_SWAP_EXACT_IN];
        let data = vec![0u8; 7 * 32];
        commands.extend_from_slice(&data);
        
        let new_amount = U256::from(500000);
        let modified = UniversalRouterParser::create_v3_swap_exact_in_with_amount(&commands, new_amount).unwrap();
        
        let extracted = UniversalRouterParser::extract_v3_swap_exact_in_amount(&modified).unwrap();
        assert_eq!(extracted, new_amount);
    }

    #[test]
    fn test_is_universal_router_execute() {
        // Create a Universal Router execute call
        let selector = UNIVERSAL_ROUTER_EXECUTE_SELECTOR;
        let params = encode(&[
            Token::Bytes(vec![V3_SWAP_EXACT_IN]),
            Token::Array(vec![Token::Bytes(vec![0u8; 32])]),
            Token::Uint(U256::from(1234567890)),
        ]);
        let calldata = [selector.to_vec(), params].concat();
        
        assert!(UniversalRouterParser::is_universal_router_execute(&calldata));
    }

    #[test]
    fn test_is_known_swap_function() {
        let exact_input_calldata = [EXACT_INPUT_SELECTOR.to_vec(), vec![0u8; 32]].concat();
        assert!(UniversalRouterParser::is_known_swap_function(&exact_input_calldata));
        
        let unknown_calldata = [vec![0x12, 0x34, 0x56, 0x78], vec![0u8; 32]].concat();
        assert!(!UniversalRouterParser::is_known_swap_function(&unknown_calldata));
    }

    #[test]
    fn test_is_command_based_encoding() {
        let command_based = [[0, 0, 0, 0].to_vec(), vec![0u8; 32]].concat();
        assert!(UniversalRouterParser::is_command_based_encoding(&command_based));
        
        let function_based = [EXACT_INPUT_SELECTOR.to_vec(), vec![0u8; 32]].concat();
        assert!(!UniversalRouterParser::is_command_based_encoding(&function_based));
    }

    #[test]
    fn test_parse_v4_swap_command() {
        // Create a V4_SWAP command with length-prefixed data
        let mut commands = vec![V4_SWAP];
        
        // Add 32 bytes for length (let's say 64 bytes of data)
        let data_length = U256::from(64);
        let mut length_bytes = [0u8; 32];
        data_length.to_big_endian(&mut length_bytes);
        commands.extend_from_slice(&length_bytes);
        
        // Add 64 bytes of dummy data
        let dummy_data = vec![0x42u8; 64];
        commands.extend_from_slice(&dummy_data);
        
        let parsed = UniversalRouterParser::parse_commands(&commands).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].command, V4_SWAP);
        assert_eq!(parsed[0].data.len(), 32 + 64); // 32 bytes length + 64 bytes data
    }

    #[test]
    fn test_parse_v4_swap_command_zero_length() {
        // Create a V4_SWAP command with zero length (should fail)
        let mut commands = vec![V4_SWAP];
        
        // Add 32 bytes for length (zero)
        let data_length = U256::from(0);
        let mut length_bytes = [0u8; 32];
        data_length.to_big_endian(&mut length_bytes);
        commands.extend_from_slice(&length_bytes);
        
        let result = UniversalRouterParser::parse_commands(&commands);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid V4_SWAP command: zero length data");
    }

    #[test]
    fn test_parse_v4_swap_command_incomplete() {
        // Create a V4_SWAP command with incomplete data
        let mut commands = vec![V4_SWAP];
        
        // Add 32 bytes for length (let's say 100 bytes of data)
        let data_length = U256::from(100);
        let mut length_bytes = [0u8; 32];
        data_length.to_big_endian(&mut length_bytes);
        commands.extend_from_slice(&length_bytes);
        
        // But only add 50 bytes of data (incomplete)
        let dummy_data = vec![0x42u8; 50];
        commands.extend_from_slice(&dummy_data);
        
        let result = UniversalRouterParser::parse_commands(&commands);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Incomplete V4_SWAP command data"));
    }

    #[test]
    fn test_extract_v4_swap_amount_not_implemented() {
        // Create a V4_SWAP command
        let mut commands = vec![V4_SWAP];
        let data_length = U256::from(64);
        let mut length_bytes = [0u8; 32];
        data_length.to_big_endian(&mut length_bytes);
        commands.extend_from_slice(&length_bytes);
        let dummy_data = vec![0x42u8; 64];
        commands.extend_from_slice(&dummy_data);
        
        let result = UniversalRouterParser::extract_v4_swap_amount(&commands);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "V4_SWAP amount extraction not yet implemented - requires V4Planner decoding");
    }

    #[test]
    fn test_create_v4_swap_with_amount_not_implemented() {
        // Create a V4_SWAP command
        let mut commands = vec![V4_SWAP];
        let data_length = U256::from(64);
        let mut length_bytes = [0u8; 32];
        data_length.to_big_endian(&mut length_bytes);
        commands.extend_from_slice(&length_bytes);
        let dummy_data = vec![0x42u8; 64];
        commands.extend_from_slice(&dummy_data);
        
        let result = UniversalRouterParser::create_v4_swap_with_amount(&commands, U256::from(1000000));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "V4_SWAP amount modification not yet implemented - requires V4Planner encoding");
    }
} 