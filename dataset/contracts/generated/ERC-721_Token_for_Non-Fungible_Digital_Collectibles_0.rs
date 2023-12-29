
#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::storage::Mapping;
use ink_lang as ink;

#[ink::contract]
mod erc721 {
    use ink::storage::{
        collections::Vec as StorageVec,
        lazy::Lazy,
    };
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    pub struct Erc721 {
        /// Mapping from token to owner.
        token_owner: Mapping<Hash, AccountId>,
        /// Mapping from owner to list of owned token IDs.
        owned_tokens: Mapping<AccountId, StorageVec<Hash>>,
        /// Mapping from token ID to index in its owner's list of owned tokens.
        owned_tokens_index: Mapping<Hash, u32>,
        /// The total number of tokens.
        total_supply: Lazy<u32>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        token_id: Hash,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        approved: AccountId,
        #[ink(topic)]
        token_id: Hash,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        TokenNotFound,
        NotTokenOwner,
        TokenAlreadyExists,
        Unauthorized,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc721 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                token_owner: Default::default(),
                owned_tokens: Default::default(),
                owned_tokens_index: Default::default(),
                total_supply: Lazy::new(0),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            *self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            self.owned_tokens.get(&owner).map(|tokens| tokens.len() as u32).unwrap_or(0)
        }

        #[ink(message)]
        pub fn owner_of(&self, token_id: Hash) -> Result<AccountId> {
            self.token_owner.get(&token_id).ok_or(Error::TokenNotFound)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, token_id: Hash) -> Result<()> {
            let caller = self.env().caller();
            self.transfer_from_to(Some(caller), Some(to), token_id)
        }

        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, token_id: Hash) -> Result<()> {
            let caller = self.env().caller();
            let owner = self.owner_of(token_id)?;
            if owner != caller {
                return Err(Error::Unauthorized);
            }
            self.env().emit_event(Approval {
                owner,
                approved: to,
                token_id,
            });
            Ok(())
        }

        fn transfer_from_to(
            &mut self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            token_id: Hash,
        ) -> Result<()> {
            let owner = self.owner_of(token_id)?;
            if Some(owner) != from {
                return Err(Error::NotTokenOwner);
            }

            if let Some(ref to_acc) = to {
                self.add_token_to(to_acc, token_id);
            }
            if let Some(ref from_acc) = from {
                self.remove_token_from(from_acc, token_id);
            }

            self.token_owner.insert(token_id, &to.unwrap());
            self.env().emit_event(Transfer { from, to, token_id });
            Ok(())
        }

        fn add_token_to(&mut self, to: &AccountId, token_id: Hash) {
            let to_tokens = self.owned_tokens.get(to).unwrap_or_default();
            let to_len = to_tokens.len() as u32;
            self.owned_tokens_index.insert(token_id, &to_len);
            self.owned_tokens.entry(*to).or_insert(StorageVec::new()).push(token_id);
        }

        fn remove_token_from(&mut self, from: &AccountId, token_id: Hash) {
            let token_index = self.owned_tokens_index.get(token_id).unwrap_or_default();
            let last_token_index = self.balance_of(*from) - 1;
            let last_token = self.owned_tokens.get(from).unwrap().get(last_token_index as usize).unwrap();

            // Swap the last token with the one being removed and pop it from the list.
            let from_tokens = self.owned_tokens.get_mut(from).unwrap();
            from_tokens.set(token_index as usize, last_token);
            from_tokens.pop();
            
            // Update the moved token's index and remove the original token index.
            self.owned_tokens_index.insert(last_token, &token_index);
            self.owned_tokens_index.remove(token_id);
        }
    }
}
