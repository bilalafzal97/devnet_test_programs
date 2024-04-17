use anchor_lang::prelude::*;

declare_id!("UAiCybB7jm6dHZBrGQotghGAHj82hZnvKZhFMjRtsBp");

#[program]
pub mod empty {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
