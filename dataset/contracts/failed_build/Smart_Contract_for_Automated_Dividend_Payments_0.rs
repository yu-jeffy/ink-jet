
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod automated_dividend_payment {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct AutomatedDividendPayment {
        /// Mapping from token holder to balance.
        balances: Mapping<AccountId, Balance>,
        /// Total token supply.
        total_supply: Balance,
        /// Mapping from token holder to withdrawn dividends.
        withdrawn_dividends: Mapping<AccountId, Balance>,
        /// Total dividends distributed.
        total_dividends: Balance,
        /// Total dividends withdrawn.
        total_withdrawn: Balance,
    }

    #[ink(event)]
    pub struct DividendDeposited {
        #[ink(topic)]
        from: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct DividendWithdrawn {
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        NoDividends,
        DividendOverflow,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl AutomatedDividendPayment {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let instance = Self {
                balances: Mapping::new(),
                total_supply,
                withdrawn_dividends: Mapping::new(),
                total_dividends: 0,
                total_withdrawn: 0,
            };
            instance.balances.insert(&caller, &total_supply);
            instance
        }

        /// Allows the contract owner to deposit dividends.
        #[ink(message, payable)]
        pub fn deposit_dividends(&mut self) -> Result<()> {
            let caller = self.env().caller();
            let deposit = self.env().transferred_balance();
            if deposit == 0 {
                return Err(Error::NoDividends);
            }
            self.total_dividends = self
                .total_dividends
                .checked_add(deposit)
                .ok_or(Error::DividendOverflow)?;
            self.env().emit_event(DividendDeposited { from: caller, value: deposit });
            Ok(())
        }

        /// Allows a token holder to withdraw their share of dividends.
        #[ink(message)]
        pub fn withdraw_dividends(&mut self) -> Result<()> {
            let caller = self.env().caller();
            let balance = self.balances.get(&caller).unwrap_or_default();
            if balance == 0 {
                return Err(Error::InsufficientBalance);
            }
            let total_withdrawable = self
                .total_dividends
                .checked_sub(self.total_withdrawn)
                .ok_or(Error::DividendOverflow)?;
            let withdrawable_dividends = balance
                .checked_mul(total_withdrawable)
                .and_then(|amount| amount.checked_div(self.total_supply))
                .ok_or(Error::DividendOverflow)?;
            let withdrawn = self.withdrawn_dividends.get(&caller).unwrap_or_default();
            let amount_to_withdraw = withdrawable_dividends
                .checked_sub(withdrawn)
                .ok_or(Error::DividendOverflow)?;
            if amount_to_withdraw == 0 {
                return Err(Error::NoDividends);
            }
            self.withdrawn_dividends.insert(&caller, &withdrawable_dividends);
            self.total_withdrawn = self
                .total_withdrawn
                .checked_add(amount_to_withdraw)
                .ok_or(Error::DividendOverflow)?;
            self.env().transfer(caller, amount_to_withdraw)?;
            self.env().emit_event(DividendWithdrawn { to: caller, value: amount_to_withdraw });
            Ok(())
        }

        /// Returns the total dividends distributed.
        #[ink(message)]
        pub fn total_dividends(&self) -> Balance {
            self.total_dividends
        }

        /// Returns the total dividends withdrawn.
        #[ink(message)]
        pub fn total_withdrawn(&self) -> Balance {
            self.total_withdrawn
        }

        /// Returns the balance of the caller.
        #[ink(message)]
        pub fn balance_of(&self) -> Balance {
            let caller = self.env().caller();
            self.balances.get(&caller).unwrap_or_default()
        }

        /// Returns the withdrawable dividends of the caller.
        #[ink(message)]
        pub fn withdrawable_dividends_of(&self) -> Result<Balance> {
            let caller = self.env().caller();
            let balance = self.balances.get(&caller).unwrap_or_default();
            let total_withdrawable = self
                .total_dividends
                .checked_sub(self.total_withdrawn)
                .ok_or(Error::DividendOverflow)?;
            let withdrawable_dividends = balance
                .checked_mul(total_withdrawable)
                .and_then(|amount| amount.checked_div(self.total_supply))
                .ok_or(Error::DividendOverflow)?;
            let withdrawn = self.withdrawn_dividends.get(&caller).unwrap_or_default();
            withdrawable_dividends
                .checked_sub(withdrawn)
                .ok_or(Error::DividendOverflow)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let contract = AutomatedDividendPayment::new(100);
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(), 100);
            assert_eq!(contract.total_dividends(), 0);
            assert_eq!(contract.total_withdrawn(), 0);
        }

        #[ink::test]
        fn deposit_dividends_works() {
            let mut contract = AutomatedDividendPayment::new(100);
            assert_eq!(contract.deposit_dividends(), Ok(()));
            assert_eq!(contract.total_dividends(), 0);
        }

        #[ink::test]
        fn withdraw_dividends_works() {
            let mut contract = AutomatedDividendPayment::new(100);
            assert_eq!(contract.deposit_dividends(), Ok(()));
            assert_eq!(contract.withdraw_dividends(), Ok(()));
            assert_eq!(contract.total_withdrawn(), 0);
        }

        #[ink::test]
        fn withdrawable_dividends_of_works() {
            let contract = AutomatedDividendPayment::new(100);
            assert_eq!(contract.withdrawable_dividends_of(), Ok(0));
        }
    }
}
