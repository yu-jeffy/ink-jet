
#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::storage::Mapping;
use ink::prelude::vec::Vec;
use ink_lang as ink;

#[ink::contract]
mod nft_marketplace {
    use super::*;

    #[ink(storage)]
    pub struct NftMarketplace {
        listings: Mapping<Hash, Listing>,
        offers: Mapping<(Hash, AccountId), Balance>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Listing {
        seller: AccountId,
        price: Balance,
    }

    #[ink(event)]
    pub struct Listed {
        #[ink(topic)]
        token_id: Hash,
        #[ink(topic)]
        seller: AccountId,
        price: Balance,
    }

    #[ink(event)]
    pub struct PriceUpdated {
        #[ink(topic)]
        token_id: Hash,
        new_price: Balance,
    }

    #[ink(event)]
    pub struct Delisted {
        #[ink(topic)]
        token_id: Hash,
    }

    #[ink(event)]
    pub struct Sold {
        #[ink(topic)]
        token_id: Hash,
        #[ink(topic)]
        seller: AccountId,
        #[ink(topic)]
        buyer: AccountId,
        price: Balance,
    }

    #[ink(event)]
    pub struct OfferPlaced {
        #[ink(topic)]
        token_id: Hash,
        #[ink(topic)]
        buyer: AccountId,
        price: Balance,
    }

    #[ink(event)]
    pub struct OfferWithdrawn {
        #[ink(topic)]
        token_id: Hash,
        #[ink(topic)]
        buyer: AccountId,
    }

    impl NftMarketplace {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                listings: Mapping::default(),
                offers: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn list_token(&mut self, token_id: Hash, price: Balance) {
            let caller = self.env().caller();
            assert!(price > 0, "Price must be greater than 0");
            let listing = Listing { seller: caller, price };
            self.listings.insert(token_id, &listing);
            self.env().emit_event(Listed {
                token_id,
                seller: caller,
                price,
            });
        }

        #[ink(message)]
        pub fn update_price(&mut self, token_id: Hash, new_price: Balance) {
            let caller = self.env().caller();
            let mut listing = self.listings.get(token_id).expect("Token not listed");
            assert_eq!(listing.seller, caller, "Only seller can update price");
            assert!(new_price > 0, "Price must be greater than 0");
            listing.price = new_price;
            self.listings.insert(token_id, &listing);
            self.env().emit_event(PriceUpdated { token_id, new_price });
        }

        #[ink(message)]
        pub fn delist_token(&mut self, token_id: Hash) {
            let caller = self.env().caller();
            let listing = self.listings.get(token_id).expect("Token not listed");
            assert_eq!(listing.seller, caller, "Only seller can delist token");
            self.listings.remove(token_id);
            self.env().emit_event(Delisted { token_id });
        }

        #[ink(message, payable)]
        pub fn buy_token(&mut self, token_id: Hash) {
            let caller = self.env().caller();
            let transferred = self.env().transferred_balance();
            let listing = self.listings.take(token_id).expect("Token not listed");
            assert!(transferred >= listing.price, "Insufficient payment");
            
            // Handle payment to the seller
            self.env().transfer(listing.seller, listing.price).expect("Transfer failed");

            // Refund any excess payment back to the buyer
            if transferred > listing.price {
                let refund = transferred - listing.price;
                self.env().transfer(caller, refund).expect("Refund failed");
            }

            self.env().emit_event(Sold {
                token_id,
                seller: listing.seller,
                buyer: caller,
                price: listing.price,
            });
        }

        #[ink(message)]
        pub fn place_offer(&mut self, token_id: Hash, offer: Balance) {
            let caller = self.env().caller();
            assert!(offer > 0, "Offer must be greater than 0");
            let listing = self.listings.get(token_id).expect("Token not listed");
            assert_ne!(listing.seller, caller, "Seller cannot place offer");
            self.offers.insert((token_id, caller), &offer);
            self.env().emit_event(OfferPlaced { token_id, buyer: caller, price: offer });
        }

        #[ink(message)]
        pub fn withdraw_offer(&mut self, token_id: Hash) {
            let caller = self.env().caller();
            self.offers.take((token_id, caller)).expect("Offer not found");
            self.env().emit_event(OfferWithdrawn { token_id, buyer: caller });
        }
    }
}
