use anchor_lang::prelude::*;


#[account]
pub struct TransactionList {
    pub list: Vec<Pubkey>,    // 50 transactions per list: 32 * 50 = 1600
    pub next_list: Option<Pubkey>,
}

impl TransactionList {
    pub const LEN: usize = 8 + 1600 + 32;        // 1640
}