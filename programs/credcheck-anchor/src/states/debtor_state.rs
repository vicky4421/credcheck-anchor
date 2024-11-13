// Shree

use anchor_lang::prelude::*;

#[account]
pub struct DebtorState {
    pub gst_registration_number: String,        // for uniqueness - 15 chars - 4 for len + 15 bytes
    pub name: String,                           // max chars - 50 - 4 for len + 50 bytes
    pub average_default_days: u32,              // 4 bytes
    pub total_transactions: u32,                // 4 bytes
    pub defaulted_transactions: Vec<Pubkey>,    // pubkey of default transactions max 10 - 10 * 32 bytes + 4 bytes for len
    pub transactions: Pubkey,                   // pubkey of first transactions list - 32 bytes
}


impl DebtorState {
    pub const LEN: usize = 8 + 19 + 54 + 4 + 4 + 324 + 32;      // 445
}