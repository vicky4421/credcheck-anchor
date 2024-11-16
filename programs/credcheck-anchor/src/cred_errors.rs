pub use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The provided transaction list is not linked to the debtor")]
    InvalidTransactionList,

    #[msg("The current transaction list is not full")]
    CurrentListIsNotFull
}