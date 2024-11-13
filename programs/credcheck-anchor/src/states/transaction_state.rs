use anchor_lang::prelude::*;

#[account]
pub struct TransactionState {
    pub debtor_pubkey: Pubkey,      // link back to debtor state - 32 bytes
    pub creditor_pubkey: Pubkey,    // creditor pubkey - 32 bytes
    pub amount: u64,                // amount - 8 bytes
    pub credit_days: u32,           // agreed credit days - 4 bytes
    pub transaction_date: i64,      // unix timestamp for last transaction - 8 bytes
    pub is_defaulted: bool,         // defaulted or not - 1 byte
    pub is_cleared: bool,           // cleared or not - 1 byte
}

impl TransactionState {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 4 + 8 + 1 + 1;     // 94
}