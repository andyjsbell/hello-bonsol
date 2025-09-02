use anchor_lang::prelude::*;

declare_id!("5aPccZnJ5XkqkEkSn55BCLGAMDcns9jqkapvzV1wagJw");

#[program]
pub mod hello_bonsol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
