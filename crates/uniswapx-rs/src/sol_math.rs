use alloy_primitives::Uint;

type U256 = Uint<256, 4>;

pub trait MulDiv {
    fn mul_div_down(&self, b: U256, c: U256) -> Result<U256, anyhow::Error>;
    fn mul_div_up(&self, b: U256, c: U256) -> Result<U256, anyhow::Error>;
}

impl MulDiv for U256 {
    fn mul_div_down(&self, b: U256, c: U256) -> Result<U256, anyhow::Error> {
        let product = self.checked_mul(b).ok_or_else(|| anyhow::anyhow!("Multiplication overflow"))?;
        product.checked_div(c).ok_or_else(|| anyhow::anyhow!("Division by zero"))
    }
    fn mul_div_up(&self, b: U256, c: U256) -> Result<U256, anyhow::Error> {
        let product = self.checked_mul(b).ok_or_else(|| anyhow::anyhow!("Multiplication overflow"))?;
        if c == U256::ZERO {
            return Err(anyhow::anyhow!("Division by zero"));
        }
        let (quotient, remainder) = product.div_rem(c);
        if remainder == U256::ZERO {
            Ok(quotient)
        } else {
            Ok(quotient + U256::from(1))
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_div_down() {
        let a = U256::from(10);
        let b = U256::from(20);
        let c = U256::from(5);
        
        assert_eq!(a.mul_div_down(b, c).unwrap(), U256::from(40));
        
        // Test with larger numbers
        let a = U256::from(1_000_000_000_000_u64);
        let b = U256::from(2_000_000_000_000_u64);
        let c = U256::from(500_000_000_000_u64);
        
        assert_eq!(a.mul_div_down(b, c).unwrap(), U256::from(4_000_000_000_000_u64));
    }

    #[test]
    fn test_mul_div_up() {
        let a = U256::from(10);
        let b = U256::from(20);
        let c = U256::from(3);
        
        assert_eq!(a.mul_div_up(b, c).unwrap(), U256::from(67));
        
        // Test with larger numbers
        let a = U256::from(1_000_000_000_000_u64);
        let b = U256::from(2_000_000_000_000_u64);
        let c = U256::from(300_000_000_000_u64);
        
        assert_eq!(a.mul_div_up(b, c).unwrap(), U256::from(6_666_666_666_667_u64));
    }

    #[test]
    fn test_mul_div_down_no_remainder() {
        let a = U256::from(10);
        let b = U256::from(20);
        let c = U256::from(2);
        
        assert_eq!(a.mul_div_down(b, c).unwrap(), U256::from(100));
    }

    #[test]
    fn test_mul_div_down_with_remainder() {
        let a = U256::from(10);
        let b = U256::from(20);
        let c = U256::from(3);
        
        assert_eq!(a.mul_div_down(b, c).unwrap(), U256::from(66));
    }

    #[test]
    fn test_mul_div_up_no_remainder() {
        let a = U256::from(10);
        let b = U256::from(20);
        let c = U256::from(2);
        
        assert_eq!(a.mul_div_up(b, c).unwrap(), U256::from(100));
    }

    #[test]
    fn test_mul_div_up_with_remainder() {
        let a = U256::from(10);
        let b = U256::from(20);
        let c = U256::from(3);
        
        assert_eq!(a.mul_div_up(b, c).unwrap(), U256::from(67));
    }

    #[test]
    fn test_mul_div_down_overflow() {
        let a = U256::MAX;
        let b = U256::from(2);
        let c = U256::from(1);
        
        let result = a.mul_div_down(b, c);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Multiplication overflow");
    }

    #[test]
    fn test_mul_div_up_overflow() {
        let a = U256::MAX;
        let b = U256::from(2);
        let c = U256::from(1);
        
        let result = a.mul_div_up(b, c);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Multiplication overflow");
    }

    #[test]
    fn test_mul_div_down_division_by_zero() {
        let a = U256::from(10);
        let b = U256::from(20);
        let c = U256::from(0);
        
        let result = a.mul_div_down(b, c);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Division by zero");
    }

    #[test]
    fn test_mul_div_up_division_by_zero() {
        let a = U256::from(10);
        let b = U256::from(20);
        let c = U256::from(0);
        
        let result = a.mul_div_up(b, c);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Division by zero");
    }
}
