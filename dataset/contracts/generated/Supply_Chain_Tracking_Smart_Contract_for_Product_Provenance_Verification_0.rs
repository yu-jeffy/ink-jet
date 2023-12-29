
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod product_tracking {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct ProductTracking {
        products: Mapping<u64, Product>,
        product_count: u64,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Product {
        id: u64,
        name: String,
        owner: AccountId,
        history: Vec<ProvenanceRecord>,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct ProvenanceRecord {
        event: String,
        timestamp: u64,
        location: String,
        responsible_party: AccountId,
    }

    impl ProductTracking {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                products: Mapping::default(),
                product_count: 0,
            }
        }

        #[ink(message)]
        pub fn add_product(&mut self, name: String) -> u64 {
            let id = self.product_count;
            let new_product = Product {
                id,
                name,
                owner: self.env().caller(),
                history: Vec::new(),
            };

            self.products.insert(&id, &new_product);
            self.product_count += 1;

            id
        }

        #[ink(message)]
        pub fn transfer_ownership(&mut self, product_id: u64, new_owner: AccountId) {
            let caller = self.env().caller();
            let mut product = self.products.get(&product_id).expect("Product does not exist");
            assert_eq!(product.owner, caller, "Only the product owner can transfer ownership");

            product.owner = new_owner;
            self.products.insert(&product_id, &product);
        }

        #[ink(message)]
        pub fn add_provenance_record(
            &mut self,
            product_id: u64,
            event: String,
            timestamp: u64,
            location: String,
        ) {
            let caller = self.env().caller();
            let mut product = self.products.get(&product_id).expect("Product does not exist");
            assert_eq!(product.owner, caller, "Only the product owner can add provenance records");

            let record = ProvenanceRecord {
                event,
                timestamp,
                location,
                responsible_party: caller,
            };

            product.history.push(record);
            self.products.insert(&product_id, &product);
        }

        #[ink(message)]
        pub fn get_product(&self, product_id: u64) -> Option<Product> {
            self.products.get(&product_id)
        }

        #[ink(message)]
        pub fn get_product_history(&self, product_id: u64) -> Option<Vec<ProvenanceRecord>> {
            self.products.get(&product_id).map(|product| product.history)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn create_contract_works() {
            let contract = ProductTracking::new();
            assert_eq!(contract.product_count, 0);
        }

        #[ink::test]
        fn add_product_works() {
            let mut contract = ProductTracking::new();
            let product_id = contract.add_product("Test Product".into());
            assert_eq!(contract.product_count, 1);
            let product = contract.get_product(product_id).unwrap();
            assert_eq!(product.name, "Test Product");
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let mut contract = ProductTracking::new();
            let accounts = ink_env::test::default_accounts::<Environment>();
            let product_id = contract.add_product("Test Product".into());

            ink_env::test::set_caller::<Environment>(accounts.alice);
            contract.transfer_ownership(product_id, accounts.bob);
            let product = contract.get_product(product_id).unwrap();
            assert_eq!(product.owner, accounts.bob);
        }

        #[ink::test]
        fn add_provenance_record_works() {
            let mut contract = ProductTracking::new();
            let accounts = ink_env::test::default_accounts::<Environment>();
            let product_id = contract.add_product("Test Product".into());

            ink_env::test::set_caller::<Environment>(accounts.alice);
            contract.add_provenance_record(product_id, "Manufactured".into(), 42, "Factory".into());
            let history = contract.get_product_history(product_id).unwrap();
            assert_eq!(history.len(), 1);
            assert_eq!(history[0].event, "Manufactured");
            assert_eq!(history[0].timestamp, 42);
            assert_eq!(history[0].location, "Factory");
            assert_eq!(history[0].responsible_party, accounts.alice);
        }
    }
}
