
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod token {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Token {
        /// Total token supply.
        total_supply: Balance,
        /// Balances map.
        balances: Mapping<AccountId, Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Token {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = Mapping::default();
            balances.insert(caller, &initial_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: caller,
                value: initial_supply,
            });
            Self {
                total_supply: initial_supply,
                balances,
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
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            self.env().emit_event(Transfer {
                from: Some(from),
                to,
                value,
            });
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;

        #[ink::test]
        fn new_works() {
            let token = Token::new(100);
            let accounts = test::default_accounts::<Environment>();
            assert_eq!(token.total_supply(), 100);
            assert_eq!(token.balance_of(accounts.alice), 100);
        }

        #[ink::test]
        fn transfer_works() {
            let mut token = Token::new(100);
            let accounts = test::default_accounts::<Environment>();
            assert_eq!(token.transfer(accounts.bob, 50), Ok(()));
            assert_eq!(token.balance_of(accounts.alice), 50);
            assert_eq!(token.balance_of(accounts.bob), 50);
        }

        #[ink::test]
        fn transfer_fails_when_insufficient_balance() {
            let mut token = Token::new(100);
            let accounts = test::default_accounts::<Environment>();
            assert_eq!(token.transfer(accounts.bob, 101), Err(Error::InsufficientBalance));
            assert_eq!(token.balance_of(accounts.alice), 100);
            assert_eq!(token.balance_of(accounts.bob), 0);
        }
    }
}
