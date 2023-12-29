
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod tokenized_real_estate {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct TokenizedRealEstate {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
        assets: Mapping<AssetId, Asset>,
        asset_ownerships: Mapping<(AssetId, AccountId), Balance>,
        next_asset_id: AssetId,
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

    #[ink(event)]
    pub struct AssetTransfer {
        #[ink(topic)]
        asset_id: AssetId,
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct AssetCreation {
        #[ink(topic)]
        asset_id: AssetId,
        owner: AccountId,
        total_shares: Balance,
    }

    #[derive(scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Asset {
        total_shares: Balance,
    }

    pub type AssetId = u32;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
        InvalidAssetId,
        Unauthorized,
        AssetAlreadyExists,
        AssetDoesNotExist,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl TokenizedRealEstate {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn create_asset(&mut self, total_shares: Balance) -> Result<AssetId> {
            let caller = self.env().caller();
            let asset_id = self.next_asset_id;
            self.next_asset_id = asset_id.checked_add(1).ok_or(Error::AssetAlreadyExists)?;
            let asset = Asset { total_shares };
            self.assets.insert(asset_id, &asset);
            self.asset_ownerships.insert((asset_id, caller), &total_shares);
            self.env().emit_event(AssetCreation {
                asset_id,
                owner: caller,
                total_shares,
            });
            Ok(asset_id)
        }

        #[ink(message)]
        pub fn transfer_asset(&mut self, asset_id: AssetId, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_asset_from_to(asset_id, &from, &to, value)
        }

        fn transfer_asset_from_to(&mut self, asset_id: AssetId, from: &AccountId, to: &AccountId, value: Balance) -> Result<()> {
            let from_balance = self.asset_ownerships.get((asset_id, from)).unwrap_or(0);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            self.asset_ownerships.insert((asset_id, from), &(from_balance - value));
            let to_balance = self.asset_ownerships.get((asset_id, to)).unwrap_or(0);
            self.asset_ownerships.insert((asset_id, to), &(to_balance + value));
            self.env().emit_event(AssetTransfer {
                asset_id,
                from: *from,
                to: *to,
                value,
            });
            Ok(())
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
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        fn transfer_from_to(&mut self, from: &AccountId, to: &AccountId, value: Balance) -> Result<()> {
            let from_balance = self.balances.get(from).unwrap_or_default();
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balances.get(to).unwrap_or_default();
            self.balances.insert(to, &(to_balance + value));
            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowances.get(&(from, caller)).unwrap_or_default();
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            self.transfer_from_to(&from, &to, value)?;
            self.allowances.insert(&(from, caller), &(allowance - value));
            Ok(())
        }
    }
}
