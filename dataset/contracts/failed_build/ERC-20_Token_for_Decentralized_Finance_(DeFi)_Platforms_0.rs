
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod defi_erc20 {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct DefiErc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, ink::prelude::scale::Encode, ink::prelude::scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl DefiErc20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = Mapping::default();
            balances.insert(caller, &total_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });
            Self {
                total_supply,
                balances,
                allowances: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert(&(owner, spender), &value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            self.transfer_from_to(from, to, value)?;
            self.allowances.insert(&(from, caller), &(allowance - value));
            Ok(())
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{
            call,
            test,
        };
        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let erc20 = DefiErc20::new(1000);
            assert_eq!(erc20.total_supply(), 1000);
        }

        #[ink::test]
        fn balance_of_works() {
            let erc20 = DefiErc20::new(1000);
            let accounts = test::default_accounts::<Environment>();
            assert_eq!(erc20.balance_of(accounts.alice), 1000);
            assert_eq!(erc20.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut erc20 = DefiErc20::new(1000);
            let accounts = test::default_accounts::<Environment>();
            assert_eq!(erc20.transfer(accounts.bob, 10), Ok(()));
            assert_eq!(erc20.balance_of(accounts.bob), 10);
        }

        #[ink::test]
        fn transfer_fails_when_insufficient_balance() {
            let mut erc20 = DefiErc20::new(1000);
            let accounts = test::default_accounts::<Environment>();
            assert_eq!(erc20.transfer(accounts.bob, 1001), Err(Error::InsufficientBalance));
        }

        #[ink::test]
        fn approval_works() {
            let mut erc20 = DefiErc20::new(1000);
            let accounts = test::default_accounts::<Environment>();
            assert_eq!(erc20.approve(accounts.bob, 200), Ok(()));
            assert_eq!(erc20.allowance(accounts.alice, accounts.bob), 200);
        }

        #[ink::test]
        fn transfer_from_works() {
            let mut erc20 = DefiErc20::new(1000);
            let accounts = test::default_accounts::<Environment>();
            erc20.approve(accounts.bob, 200).unwrap();
            test::set_caller::<Environment>(accounts.bob);
            assert_eq!(erc20.transfer_from(accounts.alice, accounts.eve, 100), Ok(()));
            assert_eq!(erc20.balance_of(accounts.eve), 100);
        }

        #[ink::test]
        fn transfer_from_fails_when_insufficient_allowance() {
            let mut erc20 = DefiErc20::new(1000);
            let accounts = test::default_accounts::<Environment>();
            erc20.approve(accounts.bob, 200).unwrap();
            test::set_caller::<Environment>(accounts.bob);
            assert_eq!(erc20.transfer_from(accounts.alice, accounts.eve, 201), Err(Error::InsufficientAllowance));
        }
    }
}
