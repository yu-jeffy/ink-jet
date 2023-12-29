
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod energy_trading {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct EnergyTrading {
        balance: Mapping<AccountId, Balance>,
        grid_balance: Balance,
        price_per_kwh: Balance,
    }

    #[ink(event)]
    pub struct Trade {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
        amount_kwh: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientGridBalance,
        TransferFailed,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl EnergyTrading {
        #[ink(constructor)]
        pub fn new(price_per_kwh: Balance) -> Self {
            Self {
                balance: Mapping::default(),
                grid_balance: 0,
                price_per_kwh,
            }
        }

        #[ink(message)]
        pub fn buy_energy(&mut self, amount_kwh: Balance) -> Result<()> {
            let buyer = self.env().caller();
            let cost = self.price_per_kwh * amount_kwh;

            let buyer_balance = self.balance_of(&buyer);
            if buyer_balance < cost {
                return Err(Error::InsufficientBalance);
            }

            if self.grid_balance < amount_kwh {
                return Err(Error::InsufficientGridBalance);
            }

            self.grid_balance -= amount_kwh;
            self.balance.insert(&buyer, &(buyer_balance - cost));

            self.env().emit_event(Trade {
                from: buyer,
                to: self.env().account_id(),
                value: cost,
                amount_kwh,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn sell_energy(&mut self, amount_kwh: Balance) -> Result<()> {
            let seller = self.env().caller();
            let revenue = self.price_per_kwh * amount_kwh;

            let seller_balance = self.balance_of(&seller);
            self.grid_balance += amount_kwh;
            self.balance.insert(&seller, &(seller_balance + revenue));

            self.env().emit_event(Trade {
                from: self.env().account_id(),
                to: seller,
                value: revenue,
                amount_kwh,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn set_price_per_kwh(&mut self, new_price: Balance) {
            let caller = self.env().caller();
            // Only the contract owner can adjust the price.
            if caller == self.env().account_id() {
                self.price_per_kwh = new_price;
            }
        }

        #[ink(message)]
        pub fn get_price_per_kwh(&self) -> Balance {
            self.price_per_kwh
        }

        #[ink(message)]
        pub fn deposit_funds(&mut self, amount: Balance) -> Result<()> {
            let caller = self.env().caller();
            let caller_balance = self.balance_of(&caller);

            self.balance.insert(&caller, &(caller_balance + amount));
            Ok(())
        }

        #[ink(message)]
        pub fn withdraw_funds(&mut self, amount: Balance) -> Result<()> {
            let caller = self.env().caller();
            let caller_balance = self.balance_of(&caller);

            if caller_balance < amount {
                return Err(Error::InsufficientBalance);
            }

            self.balance.insert(&caller, &(caller_balance - amount));
            Ok(())
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: &AccountId) -> Balance {
            self.balance.get(owner).unwrap_or_default()
        }
    }
}
