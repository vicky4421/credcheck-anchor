use anchor_lang::prelude::*;

pub mod states;
pub use states::*;

declare_id!("H2W53xLqF2FPPEMZ5aNNiYyg6PnzBQJyMXc28Jap7XGy");

#[program]
pub mod credcheck_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
