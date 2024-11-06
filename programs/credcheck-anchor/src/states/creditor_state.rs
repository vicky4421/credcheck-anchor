use anchor_lang::prelude::*;

#[account]
pub struct CreditorState {
    pub gst_registration_number: String,        // for uniqueness - 15 chars - 4 for len + 15 bytes
    pub name: String,                           // max chars - 50 - 4 for len + 50 bytes
}

impl CreditorState {
    pub const LEN: usize = 8 + 19 + 54;
}