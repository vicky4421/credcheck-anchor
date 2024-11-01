// Shree

use anchor_lang::prelude::*;

#[account]
pub struct DebtorState {
    pub gst_registration_number: String,        // for uniqueness
    pub name: String,
    pub average_default_days: u32,
    pub defaulted_to: Vec<DefaultRecord>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DefaultRecord {
    pub creditor_pubkey: Pubkey,
    pub default_days: u32,
}