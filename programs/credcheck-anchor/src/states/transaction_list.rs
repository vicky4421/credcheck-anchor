use anchor_lang::prelude::*;


#[account]
pub struct TransactionList {
    pub list: Vec<Pubkey>,              // 50 transactions per list: 32 * 50 = 1600
    pub is_full: bool,                  // 1 byte - true if list is full
    pub next_list: Option<Pubkey>,      // 32 bytes - link to next list
}

impl TransactionList {
    pub const LEN: usize = 8 + 1600 + 1 + 32;        // 1641
}