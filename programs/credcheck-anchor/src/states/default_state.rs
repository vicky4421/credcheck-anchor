use anchor_lang::prelude::*;

#[account]
pub struct DefaultState {
    pub debtor_pubkey: Pubkey,      // link back to debtor state - 32 bytes
    pub creditor_pubkey: Pubkey,    // creditor pubkey - 32 bytes
    pub default_days: u32,          // default days - 4 bytes
}


impl DefaultState {
    pub const LEN: usize = 8 + 32 + 32 + 4;
}