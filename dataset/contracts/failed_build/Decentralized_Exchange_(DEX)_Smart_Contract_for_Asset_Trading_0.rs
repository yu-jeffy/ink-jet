
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod dex {
    use ink::storage::Mapping;
    use ink_env::call::build_call;
    use ink_prelude::vec::Vec;
    use scale::{Decode, Encode};

    #[ink(storage)]
    pub struct Dex {
        assets: Mapping<AccountId, Balance>,
        orders: Mapping<(AccountId, AccountId), Order>,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Order {
        from_asset: AccountId,
        to_asset: AccountId,
        from_amount: Balance,
        to_amount: Balance,
        owner: AccountId,
    }

    #[ink(event)]
    pub struct Trade {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        from_asset: AccountId,
        #[ink(topic)]
        to_asset: AccountId,
        from_amount: Balance,
        to_amount: Balance,
    }

    #[ink(event)]
    pub struct Deposit {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        asset: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Withdraw {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        asset: AccountId,
        amount: Balance,
    }

    impl Dex {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                assets: Default::default(),
                orders: Default::default(),
            }
        }

        #[ink(message)]
        pub fn deposit(&mut self, asset: AccountId, amount: Balance) {
            let caller = self.env().caller();
            let current_balance = self.assets.get(caller).unwrap_or_default();
            self.assets.insert(caller, &(current_balance + amount));
            self.env().emit_event(Deposit {
                owner: caller,
                asset,
                amount,
            });
        }

        #[ink(message)]
        pub fn withdraw(&mut self, asset: AccountId, amount: Balance) {
            let caller = self.env().caller();
            let current_balance = self.assets.get(caller).unwrap_or_default();
            assert!(current_balance >= amount, "Not enough balance to withdraw");
            self.assets.insert(caller, &(current_balance - amount));
            self.env().emit_event(Withdraw {
                owner: caller,
                asset,
                amount,
            });
        }

        #[ink(message)]
        pub fn create_order(
            &mut self,
            from_asset: AccountId,
            to_asset: AccountId,
            from_amount: Balance,
            to_amount: Balance,
        ) {
            let caller = self.env().caller();
            let current_balance = self.assets.get(caller).unwrap_or_default();
            assert!(
                current_balance >= from_amount,
                "Not enough balance to create order"
            );
            let order = Order {
                from_asset,
                to_asset,
                from_amount,
                to_amount,
                owner: caller,
            };
            self.orders.insert((caller, to_asset), &order);
        }

        #[ink(message)]
        pub fn execute_order(&mut self, owner: AccountId, to_asset: AccountId) {
            let caller = self.env().caller();
            let order_key = (owner, to_asset);
            let order = self.orders.get(order_key).expect("Order does not exist");

            assert!(
                self.env().caller() == order.owner,
                "Only the order owner can execute the order"
            );

            let caller_balance = self.assets.get(caller).unwrap_or_default();
            assert!(
                caller_balance >= order.to_amount,
                "Not enough balance to execute order"
            );

            self.assets.insert(caller, &(caller_balance - order.to_amount));
            let owner_balance = self.assets.get(order.owner).unwrap_or_default();
            self.assets.insert(order.owner, &(owner_balance + order.to_amount));

            self.env().emit_event(Trade {
                owner: order.owner,
                from_asset: order.from_asset,
                to_asset: order.to_asset,
                from_amount: order.from_amount,
                to_amount: order.to_amount,
            });

            self.orders.remove(order_key);
        }
    }
}
