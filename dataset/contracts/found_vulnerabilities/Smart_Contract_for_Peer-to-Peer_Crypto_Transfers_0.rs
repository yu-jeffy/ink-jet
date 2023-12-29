
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod p2p_transfer {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct P2PTransfer {
        balances: Mapping<AccountId, Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
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
            Self {
                balances: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            if to == from {
                return Err(Error::TransferToSelf)
            }
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance)
            }
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            self.env().emit_event(Transfer { from, to, value });
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{test, AccountId};

        #[ink::test]
        fn new_works() {
            let p2p = P2PTransfer::new();
            assert_eq!(p2p.balance_of(AccountId::from([0x1; 32])), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut p2p = P2PTransfer::new();
            let accounts = default_accounts();
            test::set_balance::<ink::env::DefaultEnvironment>(accounts.alice, 100);
            assert_eq!(p2p.balance_of(accounts.alice), 100);
            assert_eq!(p2p.transfer(accounts.bob, 50), Ok(()));
            assert_eq!(p2p.balance_of(accounts.alice), 50);
            assert_eq!(p2p.balance_of(accounts.bob), 50);
        }

        #[ink::test]
        fn transfer_to_self_fails() {
            let mut p2p = P2PTransfer::new();
            let accounts = default_accounts();
            test::set_balance::<ink::env::DefaultEnvironment>(accounts.alice, 100);
            assert_eq!(p2p.transfer(accounts.alice, 50), Err(Error::TransferToSelf));
        }

        #[ink::test]
        fn transfer_insufficient_balance_fails() {
            let mut p2p = P2PTransfer::new();
            let accounts = default_accounts();
            assert_eq!(p2p.transfer(accounts.bob, 50), Err(Error::InsufficientBalance));
        }

        fn default_accounts() -> test::DefaultAccounts<ink::env::DefaultEnvironment> {
            test::default_accounts::<ink::env::DefaultEnvironment>()
        }
    }
}
