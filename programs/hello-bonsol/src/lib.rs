use anchor_lang::prelude::*;
use bonsol_anchor_interface::instructions::{
    execute_v1_with_accounts, CallbackConfig, ExecutionConfig,
};
use bonsol_anchor_interface::{DeployV1Account, ExecutionRequestV1Account};

declare_id!("5aPccZnJ5XkqkEkSn55BCLGAMDcns9jqkapvzV1wagJw");

const HELLO_IMAGE_ID: &str = "";

#[error_code]
pub enum HelloError {
    #[msg("Welcome Request failed")]
    WelcomeRequestFailed,
}

#[program]
pub mod hello_bonsol {
    use super::*;

    pub fn initialize(_: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn welcome(ctx: Context<Welcome>, request_id: String) -> Result<()> {
        msg!("Hello, world!");
        let requester = ctx.accounts.initiator.key();
        let payer = ctx.accounts.initiator.key();
        let execution_account = &ctx.accounts.execution_account.key();
        let deployment_account = &ctx.accounts.deployment_account.key();
        let tip = 0;
        let expire = 0;
        execute_v1_with_accounts(
            &requester,
            &payer,
            execution_account,
            deployment_account,
            HELLO_IMAGE_ID,
            &request_id,
            vec![], // Input items for program
            tip,
            expire,
            ExecutionConfig {
                verify_input_hash: true,
                input_hash: None, // Hash of input items for program, keccak256
                forward_output: true,
            },
            Some(CallbackConfig {
                program_id: crate::id(),
                instruction_prefix: vec![0],
                extra_accounts: vec![],
            }),
            None,
        )
        .map_err(|_| HelloError::WelcomeRequestFailed)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Welcome<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,
    pub execution_account: Account<'info, ExecutionRequestV1Account<'info>>,
    pub deployment_account: Account<'info, DeployV1Account<'info>>,
}

#[derive(Accounts)]
pub struct Initialize {}
