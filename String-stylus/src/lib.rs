extern crate alloc;

use stylus_sdk::{
    alloy_primitives::U256,
    prelude::*,
};


// Utility functions (can be used by other contracts)
pub mod strings {
    use stylus_sdk::alloy_primitives::U256;
    
    /// Convert U256 to decimal string (like toString in Solidity)
    pub fn to_string(value: U256) -> String {
        if value.is_zero() {
            return "0".to_string();
        }
        
        let mut v = value;
        let mut buf = Vec::new();
        
        while v > U256::ZERO {
            let digit = (v % U256::from(10)).to::<u64>() as u8;
            buf.push(b'0' + digit);
            v /= U256::from(10);
        }
        
        buf.reverse();
        String::from_utf8(buf).unwrap()
    }
    
    /// Convert U256 to hex string with 0x prefix
    pub fn to_hex_string(value: U256) -> String {
        if value.is_zero() {
            return "0x0".to_string();
        }
        
        format!("0x{:x}", value)
    }
    
    /// Convert U256 to fixed-length hex string with 0x prefix
    pub fn to_hex_string_fixed(value: U256, length: usize) -> String {
        format!("0x{:0width$x}", value, width = length)
    }
}

// Demo contract
sol_storage! {
    #[entrypoint]
    pub struct StringsDemo {
        mapping(uint256 => uint256) demo_value;
    }
}

#[public]
impl StringsDemo {
    /// Convert input to decimal string
    pub fn to_decimal_string(&self, value: U256) -> String {
        strings::to_string(value)
    }
    
    /// Convert input to hex string
    pub fn to_hex_string(&self, value: U256) -> String {
        strings::to_hex_string(value)
    }
    
    /// Convert input to fixed-length hex string
    pub fn to_hex_string_fixed(&self, value: U256, length: U256) -> String {
        let len = length.to::<usize>();
        strings::to_hex_string_fixed(value, len)
    }
    
    /// Store and retrieve a value (for demo purposes)
    pub fn store_value(&mut self, key: U256, value: U256) -> bool {
        self.demo_value.setter(key).set(value);
        true
    }
    
    pub fn get_value(&self, key: U256) -> U256 {
        self.demo_value.get(key)
    }
    
    /// Demo function that converts stored value to string
    pub fn get_value_as_string(&self, key: U256) -> String {
        let value = self.demo_value.get(key);
        strings::to_string(value)
    }
}

#[cfg(test)]
mod tests {
    use super::strings::*;
    use stylus_sdk::alloy_primitives::U256;

    #[test]
    fn test_to_string_zero() {
        assert_eq!(to_string(U256::ZERO), "0");
    }

    #[test]
    fn test_to_string_small_numbers() {
        assert_eq!(to_string(U256::from(1)), "1");
        assert_eq!(to_string(U256::from(42)), "42");
        assert_eq!(to_string(U256::from(123)), "123");
    }

    #[test]
    fn test_to_string_large_numbers() {
        assert_eq!(to_string(U256::from(1000)), "1000");
        assert_eq!(to_string(U256::from(123456789)), "123456789");
    }

    #[test]
    fn test_to_hex_string_zero() {
        assert_eq!(to_hex_string(U256::ZERO), "0x0");
    }

    #[test]
    fn test_to_hex_string_basic() {
        assert_eq!(to_hex_string(U256::from(1)), "0x1");
        assert_eq!(to_hex_string(U256::from(15)), "0xf");
        assert_eq!(to_hex_string(U256::from(16)), "0x10");
        assert_eq!(to_hex_string(U256::from(255)), "0xff");
    }

    #[test]
    fn test_to_hex_string_fixed_basic() {
        assert_eq!(to_hex_string_fixed(U256::from(1), 2), "0x01");
        assert_eq!(to_hex_string_fixed(U256::from(15), 4), "0x000f");
        assert_eq!(to_hex_string_fixed(U256::from(255), 6), "0x0000ff");
    }

    #[test]
    fn test_to_hex_string_fixed_zero() {
        assert_eq!(to_hex_string_fixed(U256::ZERO, 8), "0x00000000");
    }

    #[test]
    fn test_conversion_roundtrip() {
        let values = [0, 1, 42, 255, 1000, 123456];
        
        for &val in &values {
            let u256_val = U256::from(val);
            let decimal_str = to_string(u256_val);
            let hex_str = to_hex_string(u256_val);
            
            // Basic sanity checks
            assert!(!decimal_str.is_empty());
            assert!(hex_str.starts_with("0x"));
            assert_eq!(decimal_str, val.to_string());
        }
    }
}