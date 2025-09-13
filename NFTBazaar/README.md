# NFT Bazaar

A decentralized NFT bazaar contract written in Rust for **Arbitrum Stylus**. This program provides a trust-minimized marketplace where users can create listings, buy NFTs securely, and manage sales with integrated fee collection and administrative controls.

---

## 🔑 Key Capabilities

* **NFT Listings**: Create listings for any ERC-721 contract
* **Secure Purchases**: Buyers must send the exact payment amount
* **Flexible Fees**: Adjustable platform fees (in basis points) with safe collection
* **Listing Management**: Update prices, cancel, or force-delist items
* **Ownership Controls**: Admin-only functions for fee updates, pausing, and emergency actions
* **Event Emission**: Transparent logs for all marketplace activity
* **Optimized Deployment**: Lightweight and gas-conscious for Stylus

---

## 🏗 Contract Design

The `NFTBazaar` manages these main components:

* **Listings**: Each NFT sale entry tracks the contract address, token ID, price, lister, buyer, and status
* **Fee Logic**: Platform fee stored in basis points (1% = 100 bps, max 10%)
* **Admin Controls**: Admin can update fees, pause trading, transfer ownership, and withdraw fees
* **State Tracking**: Prevents double sales and invalid operations

---

## ⚙️ Core Functions

### Marketplace Actions

* `initialize(fee_bps)` → Deploy with an initial fee structure
* `list_nft(nft_contract, token_id, price)` → Create a new listing
* `purchase(listing_id)` → Buy a listed NFT
* `edit_price(listing_id, new_price)` → Update a listing’s price
* `cancel(listing_id)` → Cancel your own listing

### Administrative Functions

* `update_platform_fee(new_fee_bps)` → Change marketplace fee (max 10%)
* `set_paused(state)` → Pause/unpause marketplace activity
* `emergency_cancel(listing_id)` → Force cancel any listing (admin only)
* `transfer_ownership(new_admin)` → Transfer admin rights
* `withdraw_fees()` → Collect platform fees

### Read-Only Queries

* `get_listing(listing_id)` → Fetch details of a listing
* `get_active_listings()` → Retrieve all unsold listing IDs
* `get_fee_bps()` → View current platform fee
* `get_total_listings()` → Check total number of created listings

---

## 🚀 Quick Start

First install [Rust](https://www.rust-lang.org/tools/install) and the Stylus CLI tool:

```bash
cargo install --force cargo-stylus
```

Add the WebAssembly build target:

```bash
rustup target add wasm32-unknown-unknown
```

Check that the tool is working:

```bash
cargo stylus --help
```

---

## 🌐 Testnet Info

Arbitrum Stylus testnet documentation, RPC endpoints, and faucets are available [here](https://docs.arbitrum.io/stylus/reference/testnet-information).

---

## 📦 ABI Export

Generate a Solidity-compatible ABI using:

```bash
cargo stylus export-abi
```

The `export-abi` feature is already enabled in `Cargo.toml`:

```toml
[features]
export-abi = ["stylus-sdk/export-abi"]
```

---

## 📤 Deployment

Check that your contract compiles to valid WASM:

```bash
cargo stylus check
```

Deploy to testnet:

```bash
cargo stylus deploy \
    --endpoint <rpcurl> \
    --private-key <yourprivatekey> \
    --constructor-args 500
```

➡️ Example fee arguments:

* `500` → 5%
* `250` → 2.5%
* `1000` → 10% (maximum allowed)

---

## 🔒 Security Highlights

* **Strict Payment Validation**: Rejects incorrect ETH amounts
* **Access Control**: Admin-only operations for fee/ownership management
* **State Safety**: Prevents double sales or invalid cancellations
* **Fee Protection**: Ensures accurate collection and withdrawal
* **Emergency Tools**: Admin can pause trading or force cancel listings
* **Input Validation**: Checks all parameters for correctness

---

## 📢 Events

The contract emits structured logs for monitoring:

* `ListingCreated(listing_id, nft_contract, token_id, lister, price)`
* `ListingSold(listing_id, nft_contract, token_id, lister, buyer, price)`
* `PriceUpdated(listing_id, old_price, new_price)`
* `ListingCancelled(listing_id, lister)`
* `EmergencyDelisting(listing_id, admin)`
* `FeeUpdated(old_fee, new_fee)`
* `PauseToggled(paused)`
* `OwnershipTransferred(old_admin, new_admin)`

---

## 🛠 Development & Testing

### Expand Macros

The [stylus-sdk](https://github.com/OffchainLabs/stylus-sdk-rs) uses macros that expand into Rust. To inspect the expanded code:

```bash
cargo install cargo-expand
cargo expand --all-features --release --target=<YOUR_ARCHITECTURE>
```

### Recommended Test Cases

* **Listings**: Create, edit, cancel
* **Purchases**: Validate exact payments and ownership changes
* **Fees**: Verify platform fee collection and limits
* **Admin Controls**: Test pausing, fee updates, emergency cancel
* **Edge Cases**: Nonexistent listings, already sold items, invalid inputs
* **Events**: Confirm all expected logs are emitted

---

## 💡 Use Cases

The bazaar can serve as the backbone for many NFT applications:

* Digital art trading platforms
* Gaming marketplaces for in-game items
* Music and media distribution
* Domain and naming marketplaces
* Membership and utility token sales
* Multi-chain NFT hubs
