use anchor_lang::prelude::*;

declare_id!("3vmh9KsfkgmTorEqbNeGpeWaK7ZUHWkdEH55BauXuzot");

#[program]
pub mod main {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
