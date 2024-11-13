use anchor_lang::prelude::*;

use crate::states::{CreditorState, DebtorState, TransactionState};

#[derive(Accounts)]
#[instruction(debtor_gst: String, creditor_gst: String)]
pub struct RecordTransaction<'info> {
    #[account(
        init,
        payer = payer,
        space = TransactionState::LEN,
        seeds = [
            b"transaction",
            debtor.key().as_ref(),
            creditor.key().as_ref(),
            &ctx.accounts.clock.unix_timestamp.to_le_bytes(),
        ],
        bump
    )]
    pub transaction: Account<'info, TransactionState>,

    #[account(mut, seeds = [b"debtor", debtor_gst.as_bytes()], bump)]
    pub debtor: Account<'info, DebtorState>,

    #[account(mut, seeds = [b"creditor", creditor_gst.as_bytes()], bump)]
    pub creditor: Account<'info, CreditorState>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub clock: Sysvar<'info, Clock>,
}

pub fn record_transaction(
    ctx: Context<RecordTransaction>,
    debtor_gst: String,
    creditor_gst: String,
    amount: u64,
    credit_days: u32,
) -> Result<()> {
    let transaction = &mut ctx.accounts.transaction;

    // initialize fields in transaction state
    transaction.debtor_pubkey = ctx.accounts.debtor.key();
    transaction.creditor_pubkey = ctx.accounts.creditor.key();
    transaction.amount = amount;
    transaction.credit_days = credit_days;
    transaction.transaction_date = ctx.accounts.clock.unix_timestamp;
    transaction.is_defaulted = false;
    transaction.is_cleared = false;

    // update debtors last updated field
    ctx.accounts.debtor.last_updated = ctx.accounts.clock.unix_timestamp;

    Ok(())
}