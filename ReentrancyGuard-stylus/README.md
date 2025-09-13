# Reentrancy Guard for Stylus

This project implements a reentrancy protection utility for Stylus smart contracts,
inspired by OpenZeppelin's `ReentrancyGuard.sol`.

## Features

- `ReentrancyGuard` struct in Rust
- `non_reentrant` guard logic
- Safe and unsafe withdraw examples
- Simple funds tracking to demonstrate reentrancy risk

## Setup

```bash
cargo build
cargo test
