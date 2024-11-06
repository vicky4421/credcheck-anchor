use anchor_lang::prelude::*;

use crate::states::DebtorState;

#[derive(Accounts)]
#[instruction(gst_registration_number: String)]
pub struct InitializeDebtor<'info> {
    #[account(
        init,
        payer = payer,
        space = DebtorState::LEN,
        seeds = [b"debtor", gst_registration_number.as_bytes()],
        bump
    )]
    pub debtor: Account<'info, DebtorState>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_debtor(
    ctx: Context<InitializeDebtor>,
    gst_registration_number: String,
    name: String,
) -> Result<()> {
    let debtor = &mut ctx.accounts.debtor;

    // initialize fields in debtor state
    debtor.gst_registration_number = gst_registration_number;
    debtor.name = name;
    debtor.average_default_days = 0;                            // initially 0 default days
    debtor.defaulted_to = vec![];                               // initially no default transactions
    debtor.last_updated = Clock::get()?.unix_timestamp;         // initizalize to current timestamp

    Ok(())
}