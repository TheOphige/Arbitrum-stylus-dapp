# strings_utils

A utility library for converting `U256` values to string representations in Rust using the Stylus SDK.

## Functions

### `to_string(value: U256) -> String`

Converts a `U256` value to its decimal string representation.

### `to_hex_string(value: U256) -> String`

Converts a `U256` value to its hexadecimal string representation, prefixed with `0x`.

### `to_hex_string_fixed(value: U256, len: usize) -> String`

Converts a `U256` value to its hexadecimal string representation, prefixed with `0x`, and padded to the specified length.

## Usage

```rust
use strings_utils::{to_string, to_hex_string, to_hex_string_fixed};
use alloy_primitives::U256;

fn main() {
    let value = U256::from(1234567890_u64);
    println!("Decimal: {}", to_string(value));
    println!("Hex: {}", to_hex_string(value));
    println!("Fixed Hex: {}", to_hex_string_fixed(value, 64));
}
```