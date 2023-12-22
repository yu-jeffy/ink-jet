
#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink_lang as ink;

#[ink::contract]
mod lending {
    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };

    /// A Lending smart contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Lending {
        /// Mapping from borrower to the amount of token borrowed.
        borrows: Mapping<AccountId, Balance>,
        /// Mapping from lender to the amount of token lent.
        lends: Mapping<AccountId, Balance>,
    }

    /// Event emitted when a loan is taken out.
    #[ink(event)]
    pub struct LoanTaken {
        #[ink(topic)]
        borrower: AccountId,
        amount: Balance,
    }

    /// Event emitted when a loan is repaid.
    #[ink(event)]
    pub struct LoanRepaid {
        #[ink(topic)]
        borrower: AccountId,
        amount: Balance,
    }

    /// Event emitted when funds are lent to the lending pool.
    #[ink(event)]
    pub struct FundsLent {
        #[ink(topic)]
        lender: AccountId,
        amount: Balance,
    }

    /// Event emitted when a lender withdraws funds from the lending pool.
    #[ink(event)]
    pub struct FundsWithdrawn {
        #[ink(topic)]
        lender: AccountId,
        amount: Balance,
    }

    /// The Lending error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the loan amount requested is zero.
        LoanAmountZero,
        /// Returned if the borrower does not have an outstanding loan.
        NoOutstandingLoan,
        /// Returned if the repayment amount is more than the borrowed amount.
        RepaymentAmountTooLarge,
    }

    /// The Lending result type.
    pub type Result<T> = core::result::Result<T, Error>;

    impl Lending {
        /// Creates a new lending contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {})
        }

        /// Allows borrowers to take out a loan.
        #[ink(message, payable)]
        pub fn take_loan(&mut self, amount: Balance) -> Result<()> {
            let borrower = self.env().caller();
            if amount == 0 {
                return Err(Error::LoanAmountZero);
            }
            let current_borrow = self.borrows.get(&borrower).unwrap_or_default();
            self.borrows.insert(&borrower, &(current_borrow + amount));
            self.env().emit_event(LoanTaken {
                borrower,
                amount,
            });
            Ok(())
        }

        /// Allows borrowers to repay their loans.
        #[ink(message, payable)]
        pub fn repay_loan(&mut self, amount: Balance) -> Result<()> {
            let borrower = self.env().caller();
            let current_borrow = self.borrows.get(&borrower).ok_or(Error::NoOutstandingLoan)?;
            if amount > current_borrow {
                return Err(Error::RepaymentAmountTooLarge);
            }
            self.borrows.insert(&borrower, &(current_borrow - amount));
            self.env().emit_event(LoanRepaid {
                borrower,
                amount,
            });
            Ok(())
        }

        /// Allows lenders to lend money to the lending pool.
        #[ink(message, payable)]
        pub fn lend_funds(&mut self, amount: Balance) -> Result<()> {
            let lender = self.env().caller();
            let current_lend = self.lends.get(&lender).unwrap_or_default();
            self.lends.insert(&lender, &(current_lend + amount));
            self.env().emit_event(FundsLent {
                lender,
                amount,
            });
            Ok(())
        }

        /// Allows lenders to withdraw funds from the lending pool.
        #[ink(message)]
        pub fn withdraw_funds(&mut self, amount: Balance) -> Result<()> {
            let lender = self.env().caller();
            let current_lend = self.lends.get(&lender).unwrap_or_default();
            if amount > current_lend {
                return Err(Error::RepaymentAmountTooLarge);
            }
            self.lends.insert(&lender, &(current_lend - amount));
            self.env().emit_event(FundsWithdrawn {
                lender,
                amount,
            });
            Ok(())
        }
    }
}
