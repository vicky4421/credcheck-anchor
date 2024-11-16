use anchor_lang::prelude::*;

use crate::states:: {DebtorState, TransactionList};
use crate::cred_errors;

#[derive(Accounts)]
pub struct CreateNewTransactionList<'info> {
    #[account(mut)]
    pub debtor: Account<'info, DebtorState>,

    #[account(
        mut,
        constraint = current_list.key() == debtor.transactions @cred_errors::ErrorCode::InvalidTransactionList,
        constraint = current_list.is_full @cred_errors::ErrorCode::CurrentListIsNotFull
    )]
    pub current_list: Account<'info, TransactionList>,

    #[account(
        init,
        payer = payer,
        space = TransactionList::LEN,
        seeds = [b"transaction_list", debtor.key().as_ref(), &debtor.total_transactions.to_le_bytes()],
        bump,
    )]
    pub new_list: Account<'info, TransactionList>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_new_transaction_list(ctx: Context<CreateNewTransactionList>) -> Result<()> {
    let current_list = &mut ctx.accounts.current_list;
    let new_list = &mut ctx.accounts.new_list;

    // update current lists next list field to point to new list
    current_list.next_list = Some(new_list.key());

    // initialize fields in new list state
    new_list.list = Vec::with_capacity(50);
    new_list.is_full = false;
    new_list.next_list = None;

    Ok(())
}