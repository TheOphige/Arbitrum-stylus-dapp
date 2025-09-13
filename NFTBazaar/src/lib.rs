extern crate alloc;

use stylus_sdk::prelude::*;
use alloy_primitives::{U256, Address};
use alloy_sol_types::sol;

sol_storage! {
    #[entrypoint]
    pub struct NFTBazaar {
        address admin;
        uint256 platform_fee_bps;   // fee in basis points (10000 = 100%)
        uint256 listing_count;
        bool paused;
        mapping(uint256 => Listing) listings;
        mapping(uint256 => bool) finalized;
    }

    pub struct Listing {
        uint256 listing_id;
        address nft_contract;
        uint256 token_id;
        address lister;
        address buyer;
        uint256 price;
        bool sold;
    }
}

#[public]
impl NFTBazaar {
    /// Initialize bazaar with fee
    pub fn initialize(&mut self, fee_bps: U256) -> Result<(), Vec<u8>> {
        if fee_bps > U256::from(1000) { // max 10%
            return Err("Fee too high".as_bytes().to_vec());
        }
        self.admin.set(self.vm().msg_sender());
        self.platform_fee_bps.set(fee_bps);
        self.listing_count.set(U256::from(0));
        self.paused.set(false);
        Ok(())
    }

    /// Create a new NFT listing
    pub fn list_nft(
        &mut self,
        nft_contract: Address,
        token_id: U256,
        price: U256,
    ) -> Result<U256, Vec<u8>> {
        if self.paused.get() {
            return Err("Marketplace is paused".as_bytes().to_vec());
        }
        if price <= U256::from(0) {
            return Err("Price must be > 0".as_bytes().to_vec());
        }

        let new_id = self.listing_count.get() + U256::from(1);
        let sender = self.vm().msg_sender();

        let mut listing = self.listings.setter(new_id);
        listing.listing_id.set(new_id);
        listing.nft_contract.set(nft_contract);
        listing.token_id.set(token_id);
        listing.lister.set(sender);
        listing.buyer.set(Address::ZERO);
        listing.price.set(price);
        listing.sold.set(false);

        self.listing_count.set(new_id);
        self.finalized.setter(new_id).set(false);

        log(self.vm(), ListingCreated {
            listing_id: new_id,
            nft_contract,
            token_id,
            lister: sender,
            price,
        });

        Ok(new_id)
    }

    /// Purchase NFT
    pub fn purchase(&mut self, listing_id: U256) -> Result<(), Vec<u8>> {
        if self.paused.get() {
            return Err("Marketplace is paused".as_bytes().to_vec());
        }

        let (already_sold, price, nft_contract, token_id, lister) = {
            let item = self.listings.get(listing_id);
            (
                item.sold.get(),
                item.price.get(),
                item.nft_contract.get(),
                item.token_id.get(),
                item.lister.get(),
            )
        };

        if already_sold {
            return Err("Already sold".as_bytes().to_vec());
        }
        if self.vm().msg_value() != price {
            return Err("Wrong payment".as_bytes().to_vec());
        }

        let buyer = self.vm().msg_sender();
        let fee = (price * self.platform_fee_bps.get()) / U256::from(10000);
        let _seller_revenue = price - fee;

        let mut listing = self.listings.setter(listing_id);
        listing.buyer.set(buyer);
        listing.sold.set(true);

        self.finalized.setter(listing_id).set(true);

        log(self.vm(), ListingSold {
            listing_id,
            nft_contract,
            token_id,
            lister,
            buyer,
            price,
        });

        Ok(())
    }

    /// Modify listing price
    pub fn edit_price(&mut self, listing_id: U256, new_price: U256) -> Result<(), Vec<u8>> {
        let listing = self.listings.get(listing_id);

        if listing.lister.get() != self.vm().msg_sender() {
            return Err("Not your listing".as_bytes().to_vec());
        }
        if listing.sold.get() {
            return Err("Already sold".as_bytes().to_vec());
        }
        if new_price <= U256::from(0) {
            return Err("Invalid price".as_bytes().to_vec());
        }

        let old_price = listing.price.get();
        self.listings.setter(listing_id).price.set(new_price);

        log(self.vm(), PriceUpdated {
            listing_id,
            old_price,
            new_price,
        });

        Ok(())
    }

    /// Cancel listing (seller only)
    pub fn cancel(&mut self, listing_id: U256) -> Result<(), Vec<u8>> {
        let listing = self.listings.get(listing_id);

        if listing.lister.get() != self.vm().msg_sender() {
            return Err("Not your listing".as_bytes().to_vec());
        }
        if listing.sold.get() {
            return Err("Already sold".as_bytes().to_vec());
        }

        self.listings.setter(listing_id).sold.set(true);
        self.finalized.setter(listing_id).set(true);

        log(self.vm(), ListingCancelled {
            listing_id,
            lister: self.vm().msg_sender(),
        });

        Ok(())
    }

    /// ADMIN: force-cancel listing
    pub fn emergency_cancel(&mut self, listing_id: U256) -> Result<(), Vec<u8>> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err("Only admin".as_bytes().to_vec());
        }

        self.listings.setter(listing_id).sold.set(true);
        self.finalized.setter(listing_id).set(true);

        log(self.vm(), EmergencyDelisting {
            listing_id,
            admin: self.vm().msg_sender(),
        });

        Ok(())
    }

    /// ADMIN: change fee
    pub fn update_platform_fee(&mut self, new_fee_bps: U256) -> Result<(), Vec<u8>> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err("Only admin".as_bytes().to_vec());
        }
        if new_fee_bps > U256::from(1000) {
            return Err("Too high".as_bytes().to_vec());
        }

        let old = self.platform_fee_bps.get();
        self.platform_fee_bps.set(new_fee_bps);

        log(self.vm(), FeeUpdated {
            old_fee: old,
            new_fee: new_fee_bps,
        });

        Ok(())
    }

    /// ADMIN: pause/unpause
    pub fn set_paused(&mut self, state: bool) -> Result<(), Vec<u8>> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err("Only admin".as_bytes().to_vec());
        }
        self.paused.set(state);

        log(self.vm(), PauseToggled {
            paused: state,
        });

        Ok(())
    }

    /// ADMIN: transfer ownership
    pub fn transfer_ownership(&mut self, new_admin: Address) -> Result<(), Vec<u8>> {
        if self.vm().msg_sender() != self.admin.get() {
            return Err("Only admin".as_bytes().to_vec());
        }
        self.admin.set(new_admin);

        log(self.vm(), OwnershipTransferred {
            old_admin: self.vm().msg_sender(),
            new_admin,
        });

        Ok(())
    }

    /// View listing
    pub fn get_listing(&self, listing_id: U256) -> (U256, Address, U256, Address, Address, U256, bool) {
        let l = self.listings.get(listing_id);
        (
            l.listing_id.get(),
            l.nft_contract.get(),
            l.token_id.get(),
            l.lister.get(),
            l.buyer.get(),
            l.price.get(),
            l.sold.get(),
        )
    }

    /// Get active (unsold) listings count
    pub fn get_active_listings(&self) -> Vec<U256> {
        let mut ids = Vec::new();
        let total = self.listing_count.get();
        let mut i = U256::from(1);
        while i <= total {
            let item = self.listings.get(i);
            if !item.sold.get() {
                ids.push(i);
            }
            i = i + U256::from(1);
        }
        ids
    }

    pub fn get_fee_bps(&self) -> U256 {
        self.platform_fee_bps.get()
    }

    pub fn get_total_listings(&self) -> U256 {
        self.listing_count.get()
    }
}

sol! {
    event ListingCreated(
        uint256 indexed listing_id,
        address indexed nft_contract,
        uint256 indexed token_id,
        address lister,
        uint256 price
    );
    event ListingSold(
        uint256 indexed listing_id,
        address indexed nft_contract,
        uint256 indexed token_id,
        address lister,
        address buyer,
        uint256 price
    );
    event PriceUpdated(uint256 indexed listing_id, uint256 old_price, uint256 new_price);
    event ListingCancelled(uint256 indexed listing_id, address indexed lister);
    event EmergencyDelisting(uint256 indexed listing_id, address indexed admin);
    event FeeUpdated(uint256 old_fee, uint256 new_fee);
    event PauseToggled(bool paused);
    event OwnershipTransferred(address indexed old_admin, address indexed new_admin);
}