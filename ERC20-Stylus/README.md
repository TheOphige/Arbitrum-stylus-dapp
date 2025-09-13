# 🪙 Stylus ERC-20 Token

## 📖 Overview

This project implements the **ERC-20 token standard** in [Stylus](https://docs.arbitrum.io/stylus), written in **Rust**. It is a learning-oriented implementation that mimics the OpenZeppelin ERC-20 contract, but adapted for the Stylus smart contract framework.

The contract provides the standard ERC-20 functionality:

* Token metadata (`name`, `symbol`, `decimals`, `totalSupply`)
* Balance tracking (`balanceOf`)
* Transfers (`transfer`, `transferFrom`)
* Allowance system (`approve`, `allowance`)
* Event logs (`Transfer`, `Approval`)

---

## 📂 Project Structure

```
erc20_token/
├── src/
│   ├── lib.rs         # Main contract entrypoint
│   └── erc20.rs       # ERC-20 implementation logic
├── tests/
│   └── test_erc20.rs       # Unit & integration tests
├── README.md          # Project documentation
└── Cargo.toml         # Rust project config
```

---

## ⚙️ Features

* ✅ Implements all required ERC-20 functions
* ✅ Events for `Transfer` and `Approval`
* ✅ Unit tests for transfers, approvals, and failure cases
* ✅ Constructor-based minting of initial supply
* ⬜ (Optional) `mint` function restricted to owner

---

## 🚀 Getting Started

### 1. Install dependencies

Ensure you have Rust and Stylus installed.

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-stylus
```

### 2. Create project (already done here)

```bash
cargo stylus new erc20_token
cd erc20_token
```

### 3. Build contract

```bash
cargo stylus build
```

This compiles the contract into **WASM** for Stylus deployment.

### 4. Run tests

```bash
cargo test
```

Runs Rust-based unit tests for the ERC-20 logic.

---

## 📜 Example Usage

* **Deploy contract** with initial supply minted to the deployer.
* **Transfer tokens**:

  ```rust
  erc20.transfer(alice, U256::from(100));
  ```
* **Approve allowance**:

  ```rust
  erc20.approve(bob, U256::from(50));
  ```
* **Transfer using allowance**:

  ```rust
  erc20.transfer_from(alice, carol, U256::from(30));
  ```

---

## 🧪 Tests Included

* ✅ `balanceOf` returns correct balances
* ✅ `transfer` reduces sender and increases receiver balances
* ✅ `approve` sets allowance correctly
* ✅ `transferFrom` respects allowances
* ✅ Reverts on insufficient balance or allowance

---

## 📚 References

* [ERC-20 Standard (EIP-20)](https://eips.ethereum.org/EIPS/eip-20)
* [OpenZeppelin ERC-20](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol)
* [Stylus Documentation](https://docs.arbitrum.io/stylus)

---

## ✅ Submission Format

**GitHub Repo:** [https://github.com/YOUR\_USERNAME/stylus-erc20](https://github.com/YOUR_USERNAME/stylus-erc20)

**Summary:**
Implemented the ERC-20 token standard in Rust for Stylus. Includes all required functions, events, and tests.

**Commands:**

```bash
cargo stylus build
cargo test
```
