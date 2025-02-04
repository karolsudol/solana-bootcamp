use anchor_lang::prelude::*;

declare_id!("2vBz5hPhDTrH3YqQ69RvU74vMyeynvJTvocZVCpp6LxY");

#[program]
pub mod program_0_anchor_basics {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
