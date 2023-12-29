
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod insurance_claim {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct InsuranceClaim {
        claims: Mapping<u64, ClaimData>,
        total_claims: u64,
        approved_claims: Mapping<u64, bool>,
    }

    #[ink(event)]
    pub struct ClaimCreated {
        #[ink(topic)]
        claim_id: u64,
        account: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct ClaimApproved {
        #[ink(topic)]
        claim_id: u64,
    }

    #[ink(event)]
    pub struct ClaimRejected {
        #[ink(topic)]
        claim_id: u64,
    }

    #[derive(scale::Encode, scale::Decode, Clone, PartialEq, Eq, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct ClaimData {
        account: AccountId,
        amount: Balance,
        is_approved: bool,
    }

    impl InsuranceClaim {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn create_claim(&mut self, amount: Balance) -> u64 {
            let caller = self.env().caller();
            let claim_id = self.total_claims;
            let claim_data = ClaimData {
                account: caller,
                amount,
                is_approved: false,
            };

            self.claims.insert(claim_id, &claim_data);
            self.total_claims += 1;

            self.env().emit_event(ClaimCreated {
                claim_id,
                account: caller,
                amount,
            });

            claim_id
        }

        #[ink(message)]
        pub fn approve_claim(&mut self, claim_id: u64) {
            self.ensure_manager();
            let mut claim = self.claims.get(claim_id).expect("Claim does not exist");
            assert!(!claim.is_approved, "Claim is already approved");

            claim.is_approved = true;
            self.approved_claims.insert(claim_id, &true);
            self.claims.insert(claim_id, &claim);

            self.env().emit_event(ClaimApproved { claim_id });
        }

        #[ink(message)]
        pub fn reject_claim(&mut self, claim_id: u64) {
            self.ensure_manager();
            let claim = self.claims.get(claim_id).expect("Claim does not exist");
            assert!(!claim.is_approved, "Claim is already approved");

            self.claims.remove(claim_id);

            self.env().emit_event(ClaimRejected { claim_id });
        }

        #[ink(message)]
        pub fn get_claim(&self, claim_id: u64) -> Option<ClaimData> {
            self.claims.get(claim_id)
        }

        #[ink(message)]
        pub fn is_claim_approved(&self, claim_id: u64) -> bool {
            self.approved_claims.get(claim_id).unwrap_or(false)
        }

        fn ensure_manager(&self) {
            let caller = self.env().caller();
            let manager = self.env().account_id();
            assert_eq!(caller, manager, "Caller is not the manager");
        }
    }
}
