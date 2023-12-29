
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod digital_identity {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct DigitalIdentity {
        identities: Mapping<AccountId, IdentityInfo>,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct IdentityInfo {
        verified: bool,
        data_hash: Hash,
    }

    #[ink(event)]
    pub struct IdentityRegistered {
        #[ink(topic)]
        account: AccountId,
        data_hash: Hash,
    }

    #[ink(event)]
    pub struct IdentityVerified {
        #[ink(topic)]
        account: AccountId,
    }

    #[ink(event)]
    pub struct IdentityRevoked {
        #[ink(topic)]
        account: AccountId,
    }

    impl DigitalIdentity {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn register_identity(&mut self, data_hash: Hash) -> bool {
            let caller = self.env().caller();
            let is_existing_identity = self.identities.contains(caller);
            if is_existing_identity {
                return false
            }

            self.identities.insert(caller, &IdentityInfo {
                verified: false,
                data_hash,
            });

            self.env().emit_event(IdentityRegistered {
                account: caller,
                data_hash,
            });

            true
        }

        #[ink(message)]
        pub fn verify_identity(&mut self, account: AccountId) -> bool {
            let mut identity = self.identities.get(account).unwrap_or_default();
            if identity.verified {
                return false
            }

            identity.verified = true;
            self.identities.insert(account, &identity);

            self.env().emit_event(IdentityVerified {
                account,
            });

            true
        }

        #[ink(message)]
        pub fn revoke_identity(&mut self, account: AccountId) -> bool {
            let identity = self.identities.take(account);
            if identity.is_none() {
                return false
            }

            self.env().emit_event(IdentityRevoked {
                account,
            });

            true
        }

        #[ink(message)]
        pub fn get_identity(&self, account: AccountId) -> Option<IdentityInfo> {
            self.identities.get(account)
        }

        #[ink(message)]
        pub fn is_identity_verified(&self, account: AccountId) -> bool {
            self.identities.get(account).map(|info| info.verified).unwrap_or(false)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;
        use ink_lang as ink;

        #[ink::test]
        fn new_creates_contract() {
            let contract = DigitalIdentity::new();
            assert_eq!(contract.get_identity(AccountId::from([0x01; 32])), None);
        }

        #[ink::test]
        fn register_and_verify_identity() {
            let mut contract = DigitalIdentity::new();
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            assert!(contract.register_identity(Hash::from([0x01; 32])));
            assert!(!contract.is_identity_verified(accounts.alice));
            assert!(contract.verify_identity(accounts.alice));
            assert!(contract.is_identity_verified(accounts.alice));
            assert!(!contract.verify_identity(accounts.alice)); // already verified
        }

        #[ink::test]
        fn revoke_identity_works() {
            let mut contract = DigitalIdentity::new();
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            assert!(contract.register_identity(Hash::from([0x01; 32])));
            assert!(contract.revoke_identity(accounts.alice));
            assert!(!contract.is_identity_verified(accounts.alice));
            assert!(!contract.revoke_identity(accounts.alice)); // already revoked
        }
    }
}
