use anchor_lang::prelude::*;

use crate::{states::DebtorState, TransactionList};

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

    #[account(
        init,
        payer = payer,
        space = TransactionList::LEN,
        seeds = [b"transaction_list", debtor.key().as_ref()],
        bump
    )]
    pub transaction_list: Account<'info, TransactionList>,

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
    let transaction_list = &mut ctx.accounts.transaction_list;

    // initialize fields in debtor state
    debtor.gst_registration_number = gst_registration_number;
    debtor.name = name;
    debtor.average_default_days = 0;                            // initially 0 default days
    debtor.total_transactions = 0;                              // initially no transactions
    debtor.defaulted_transactions = Vec::with_capacity(10);     // new empty vector for defaulted transactions
    debtor.transactions = transaction_list.key();               // link back to transaction list
           
    // initialize fields in transaction list state
    transaction_list.list = Vec::with_capacity(50);
    transaction_list.is_full = false;
    transaction_list.next_list = None;

    Ok(())
}   