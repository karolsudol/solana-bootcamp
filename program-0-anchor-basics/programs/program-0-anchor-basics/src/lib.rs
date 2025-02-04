use anchor_lang::prelude::*;

declare_id!("HXoU4vHC2TNxqd4yUT1u7ppsQQA23fL2PMAX2TeMAsAw");

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
