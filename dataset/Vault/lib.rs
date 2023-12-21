
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod vault {
    use ink::storage::Mapping;
    use ink_prelude::vec::Vec;

    #[ink(storage)]
    pub struct Vault {
        // Stores a mapping from each user to their balance
        balances: Mapping<AccountId, Balance>,
        // The total balance managed by the vault
        total_balance: Balance,
    }

    #[ink(event)]
    pub struct Deposit {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    #[ink(event)]
    pub struct Withdraw {
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    impl Vault {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                balances: Mapping::new(),
                total_balance: 0,
            }
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) {
            let caller = self.env().caller();
            let value = self.env().transferred_balance();
            let caller_balance = self.balances.get(&caller).unwrap_or(0);
            self.balances.insert(&caller, &(caller_balance + value));
            self.total_balance += value;
            self.env().emit_event(Deposit {
                from: caller,
                amount: value,
            });
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) -> Result<(), ()> {
            let caller = self.env().caller();
            let caller_balance = self.balances.get(&caller).unwrap_or(0);
            if amount > caller_balance {
                return Err(())
            }
            self.balances.insert(&caller, &(caller_balance - amount));
            self.total_balance -= amount;
            self.env().transfer(caller, amount).map_err(|_| ())?;
            self.env().emit_event(Withdraw {
                to: caller,
                amount: amount,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn get_balance(&self, user: AccountId) -> Balance {
            self.balances.get(&user).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_total_balance(&self) -> Balance {
            self.total_balance
        }
    }
}
