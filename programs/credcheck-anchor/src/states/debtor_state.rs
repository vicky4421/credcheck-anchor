// Shree

use anchor_lang::prelude::*;

use crate::states::transaction_state::TransactionState;

#[account]
pub struct DebtorState {
    pub gst_registration_number: String,        // for uniqueness - 15 chars - 4 for len + 15 bytes
    pub name: String,                           // max chars - 50 - 4 for len + 50 bytes
    pub average_default_days: u32,              // 4 bytes
    pub defaulted_to: Vec<TransactionState>,    // pubkey of default transactions max 10 - 10 * 93 bytes + 4 bytes for len
    pub last_updated: i64,                      // unix timestamp for last transaction - 8 bytes
}


impl DebtorState {
    pub const LEN: usize = 8 + 19 + 54 + 4 + 934 + 8;
}