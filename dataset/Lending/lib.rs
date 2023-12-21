
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod lending {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Lending {
        loans: Mapping<AccountId, Balance>,
        interest_rate: u128,
    }

    #[ink(event)]
    pub struct LoanCreated {
        #[ink(topic)]
        borrower: AccountId,
        amount: Balance,
        interest_rate: u128,
    }

    #[ink(event)]
    pub struct LoanRepaid {
        #[ink(topic)]
        borrower: AccountId,
        amount_repaid: Balance,
    }

    #[ink(event)]
    pub struct LoanDefaulted {
        #[ink(topic)]
        borrower: AccountId,
        amount_defaulted: Balance,
    }

    impl Lending {
        #[ink(constructor)]
        pub fn new(interest_rate: u128) -> Self {
            Self {
                loans: Default::default(),
                interest_rate,
            }
        }

        #[ink(message)]
        pub fn create_loan(&mut self, borrower: AccountId, amount: Balance) {
            self.loans.insert(borrower, &amount);
            self.env().emit_event(LoanCreated {
                borrower,
                amount,
                interest_rate: self.interest_rate,
            });
        }

        #[ink(message)]
        pub fn repay_loan(&mut self, borrower: AccountId, amount: Balance) {
            let loan_amount = self.loans.get(borrower).unwrap_or(0);
            if amount >= loan_amount {
                self.loans.remove(borrower);
                self.env().emit_event(LoanRepaid {
                    borrower,
                    amount_repaid: loan_amount,
                });
            } else {
                self.loans.insert(borrower, &(loan_amount - amount));
            }
        }

        #[ink(message)]
        pub fn default_loan(&mut self, borrower: AccountId) {
            let defaulted_amount = self.loans.get(borrower).unwrap_or(0);
            self.loans.remove(borrower);
            self.env().emit_event(LoanDefaulted {
                borrower,
                amount_defaulted: defaulted_amount,
            });
        }

        #[ink(message)]
        pub fn get_loan_amount(&self, borrower: AccountId) -> Balance {
            self.loans.get(borrower).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_interest_rate(&self) -> u128 {
            self.interest_rate
        }
    }
}
