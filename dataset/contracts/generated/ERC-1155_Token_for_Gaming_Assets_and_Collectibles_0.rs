
#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink_lang as ink;

#[ink::contract]
mod erc1155 {
    use ink::storage::Mapping;
    use scale::{Decode, Encode};

    #[ink(storage)]
    pub struct Erc1155 {
        balances: Mapping<(AccountId, u32), u64>,
        approvals: Mapping<(AccountId, AccountId), bool>,
    }

    #[ink(event)]
    pub struct TransferSingle {
        #[ink(topic)]
        operator: AccountId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        id: u32,
        value: u64,
    }

    #[ink(event)]
    pub struct TransferBatch {
        #[ink(topic)]
        operator: AccountId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        ids: Vec<u32>,
        values: Vec<u64>,
    }

    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        ApprovalMissing,
        ArrayLengthMismatch,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc1155 {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Default::default(),
                approvals: Default::default(),
            }
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId, id: u32) -> u64 {
            self.balances.get((owner, id)).unwrap_or(0)
        }

        #[ink(message)]
        pub fn balance_of_batch(
            &self,
            owners: Vec<AccountId>,
            ids: Vec<u32>,
        ) -> Vec<u64> {
            let mut balances = Vec::new();
            for (owner, id) in owners.iter().zip(ids.iter()) {
                balances.push(self.balance_of(*owner, *id));
            }
            balances
        }

        #[ink(message)]
        pub fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) {
            let sender = self.env().caller();
            self.approvals.insert((sender, operator), &approved);
            self.env().emit_event(ApprovalForAll {
                owner: sender,
                operator,
                approved,
            });
        }

        #[ink(message)]
        pub fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            self.approvals.get((owner, operator)).unwrap_or(false)
        }

        #[ink(message)]
        pub fn safe_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: u32,
            value: u64,
        ) -> Result<()> {
            let operator = self.env().caller();
            if from != operator && !self.is_approved_for_all(from, operator) {
                return Err(Error::ApprovalMissing);
            }
            let from_balance = self.balance_of(from, id);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert((from, id), &(from_balance - value));
            let to_balance = self.balance_of(to, id);
            self.balances.insert((to, id), &(to_balance + value));
            self.env().emit_event(TransferSingle {
                operator,
                from,
                to,
                id,
                value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn safe_batch_transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            ids: Vec<u32>,
            values: Vec<u64>,
        ) -> Result<()> {
            let operator = self.env().caller();
            if from != operator && !self.is_approved_for_all(from, operator) {
                return Err(Error::ApprovalMissing);
            }
            if ids.len() != values.len() {
                return Err(Error::ArrayLengthMismatch);
            }
            for (id, value) in ids.iter().zip(values.iter()) {
                let from_balance = self.balance_of(from, *id);
                if from_balance < *value {
                    return Err(Error::InsufficientBalance);
                }
                self.balances.insert((from, *id), &(from_balance - value));
                let to_balance = self.balance_of(to, *id);
                self.balances.insert((to, *id), &(to_balance + value));
            }
            self.env().emit_event(TransferBatch {
                operator,
                from,
                to,
                ids,
                values,
            });
            Ok(())
        }
    }
}
