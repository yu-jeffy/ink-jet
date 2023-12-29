
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod governance_token {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct GovernanceToken {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
        owner: AccountId,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
        OnlyOwnerAllowed,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl GovernanceToken {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut instance = Self {
                total_supply,
                balances: Mapping::default(),
                allowances: Mapping::default(),
                owner: caller,
            };
            instance.balances.insert(caller, &total_supply);
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });
            instance
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), &value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            self.transfer_from_to(from, to, value)?;
            self.allowances.insert((from, caller), &(allowance - value));
            Ok(())
        }

        fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
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

        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, value: Balance) -> Result<()> {
            self.only_owner()?;
            let total_supply = self.total_supply;
            self.total_supply = total_supply.checked_add(value).ok_or(Error::InsufficientBalance)?;
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            self.env().emit_event(Transfer {
                from: None,
                to: Some(to),
                value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn burn(&mut self, from: AccountId, value: Balance) -> Result<()> {
            self.only_owner()?;
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from, &(from_balance - value));
            let total_supply = self.total_supply;
            self.total_supply = total_supply.checked_sub(value).ok_or(Error::InsufficientBalance)?;
            self.env().emit_event(Transfer {
                from: Some(from),
                to: None,
                value,
            });
            Ok(())
        }

        fn only_owner(&self) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::OnlyOwnerAllowed);
            }
            Ok(())
        }
    }
}
