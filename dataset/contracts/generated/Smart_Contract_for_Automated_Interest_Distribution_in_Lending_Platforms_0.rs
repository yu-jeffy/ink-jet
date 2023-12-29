
#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::storage::Mapping;
use ink_lang as ink;

#[ink::contract]
mod interest_distribution {
    use ink::storage::collections::Vec as StorageVec;
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct InterestDistribution {
        total_lent: Balance,
        lenders: StorageVec<AccountId>,
        balances: Mapping<AccountId, Balance>,
        interest_rate: u128,
        last_update: Timestamp,
    }

    #[ink(event)]
    pub struct InterestAccrued {
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        interest: Balance,
    }

    #[ink(event)]
    pub struct InterestWithdrawn {
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    impl InterestDistribution {
        #[ink(constructor)]
        pub fn new(interest_rate: u128) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.interest_rate = interest_rate;
                contract.last_update = Self::env().block_timestamp();
            })
        }

        #[ink(message)]
        pub fn lend(&mut self, amount: Balance) -> bool {
            let caller = self.env().caller();
            self.update_interest(caller);
            let new_balance = self.balances.get(caller).unwrap_or_default() + amount;
            self.balances.insert(caller, &new_balance);
            if !self.is_lender(&caller) {
                self.lenders.push(caller);
            }
            self.total_lent += amount;
            true
        }

        #[ink(message)]
        pub fn withdraw_interest(&mut self) -> Balance {
            let caller = self.env().caller();
            self.update_interest(caller);
            let interest = self.balances.get(caller).unwrap_or_default();
            self.balances.insert(caller, &0);
            self.env().emit_event(InterestWithdrawn {
                account: caller,
                amount: interest,
            });
            interest
        }

        fn update_interest(&mut self, account: AccountId) {
            let now = self.env().block_timestamp();
            if now > self.last_update {
                let time_diff = now - self.last_update;
                self.last_update = now;
                let mut balance = self.balances.get(account).unwrap_or_default();

                for lender in self.lenders.iter() {
                    let lender_balance = self.balances.get(lender).unwrap_or_default();
                    let interest = self.calculate_interest(lender_balance, time_diff);
                    balance += interest;
                    self.balances.insert(lender, &balance);
                    self.env().emit_event(InterestAccrued {
                        account: *lender,
                        interest,
                    });
                }
            }
        }

        fn calculate_interest(&self, balance: Balance, time_diff: Timestamp) -> Balance {
            (balance * self.interest_rate as Balance / 100) * time_diff / (365 * 24 * 60 * 60)
        }

        fn is_lender(&self, account: &AccountId) -> bool {
            self.lenders.iter().any(|lender| lender == *account)
        }

        #[ink(message)]
        pub fn get_balance(&self, account: AccountId) -> Balance {
            self.balances.get(account).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_total_lent(&self) -> Balance {
            self.total_lent
        }

        #[ink(message)]
        pub fn get_interest_rate(&self) -> u128 {
            self.interest_rate
        }
    }
}
