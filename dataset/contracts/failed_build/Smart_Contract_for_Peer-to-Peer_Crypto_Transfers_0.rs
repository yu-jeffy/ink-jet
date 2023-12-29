
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod p2p_transfer {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct P2PTransfer {
        balances: Mapping<AccountId, Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        TransferToSelf,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl P2PTransfer {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            if to == from {
                return Err(Error::TransferToSelf);
            }
            let from_balance = self.balances.get(&from).unwrap_or_default();
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(&from, &(from_balance - value));
            let to_balance = self.balances.get(&to).unwrap_or_default();
            self.balances.insert(&to, &(to_balance + value));
            self.env().emit_event(Transfer { from, to, value });
            Ok(())
        }

        #[ink(message)]
        pub fn get_balance(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn deposit(&mut self) -> Result<()> {
            let caller = self.env().caller();
            let value = self.env().transferred_value();
            let balance = self.balances.get(&caller).unwrap_or_default();
            self.balances.insert(&caller, &(balance + value));
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{
            test,
            CallData,
            CreateParams,
            Environment,
        };
        use ink::primitives::H256;
        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let contract = P2PTransfer::new();
            assert_eq!(contract.get_balance(AccountId::from([0x0; 32])), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = P2PTransfer::new();
            let accounts = default_accounts();

            // Simulate a deposit to the contract
            test::push_execution_context::<Environment>(
                accounts.alice,
                contract_id(),
                1000000,
                100,
                CallData::new(ink::selector_bytes!("deposit")),
            );
            assert_eq!(contract.deposit(), Ok(()));
            assert_eq!(contract.get_balance(accounts.alice), 100);

            // Alice transfers 10 to Bob.
            assert_eq!(contract.transfer(accounts.bob, 10), Ok(()));
            assert_eq!(contract.get_balance(accounts.alice), 90);
            assert_eq!(contract.get_balance(accounts.bob), 10);
        }

        #[ink::test]
        fn transfer_insufficient_balance() {
            let mut contract = P2PTransfer::new();
            let accounts = default_accounts();

            // Simulate a deposit to the contract
            test::push_execution_context::<Environment>(
                accounts.alice,
                contract_id(),
                1000000,
                10,
                CallData::new(ink::selector_bytes!("deposit")),
            );
            assert_eq!(contract.deposit(), Ok(()));
            assert_eq!(contract.get_balance(accounts.alice), 10);

            // Alice has insufficient funds to transfer 100 to Bob.
            assert_eq!(contract.transfer(accounts.bob, 100), Err(Error::InsufficientBalance));
            assert_eq!(contract.get_balance(accounts.alice), 10);
            assert_eq!(contract.get_balance(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_to_self_fails() {
            let mut contract = P2PTransfer::new();
            let accounts = default_accounts();

            // Simulate a deposit to the contract
            test::push_execution_context::<Environment>(
                accounts.alice,
                contract_id(),
                1000000,
                100,
                CallData::new(ink::selector_bytes!("deposit")),
            );
            assert_eq!(contract.deposit(), Ok(()));
            assert_eq!(contract.get_balance(accounts.alice), 100);

            // Alice cannot transfer to herself.
            assert_eq!(contract.transfer(accounts.alice, 10), Err(Error::TransferToSelf));
            assert_eq!(contract.get_balance(accounts.alice), 100);
        }

        fn default_accounts() -> test::DefaultAccounts<Environment> {
            test::default_accounts::<Environment>()
        }

        fn contract_id() -> AccountId {
            AccountId::from(H256::repeat_byte(0x01))
        }
    }
}
