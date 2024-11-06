use anchor_lang::prelude::*;

use crate::states::CreditorState;

#[derive(Accounts)]
#[instruction(gst_registration_number: String)]
pub struct InitializeCreditor<'info> {
    #[account(
        init,
        payer = payer,
        space = CreditorState::LEN,
        seeds = [b"creditor", gst_registration_number.as_bytes()],
        bump
    )]
    pub creditor: Account<'info, CreditorState>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_creditor(
    ctx: Context<InitializeCreditor>,
    gst_registration_number: String,
    name: String
) -> Result<()>{
    let creditor = &mut ctx.accounts.creditor;

    // initialize fields
    creditor.gst_registration_number = gst_registration_number;
    creditor.name = name;

    Ok(())
}