use anchor_lang::prelude::*;
use bonsol_anchor_interface::instructions::{
    execute_v1_with_accounts, CallbackConfig, ExecutionConfig,
};
use bonsol_anchor_interface::{DeployV1Account, ExecutionRequestV1Account};

declare_id!("4PAptfatXKk8ctGCJhrq6hbcujmASkYnG5BpshioRaug");

const WELCOME_IMAGE_ID: &str = "a6c375eb8ad1dea5ced1eb999c3c3f465b739b6683b2ca48df4c9a9834ff93f7";

#[error_code]
pub enum WelcomeError {
    #[msg("Welcome Request failed")]
    WelcomeRequestFailed,
    #[msg("Invalid callback")]
    InvalidCallback,
    #[msg("Callback failed")]
    CallbackFailed,
}

#[program]
pub mod hello_bonsol {
    use anchor_lang::solana_program::keccak;
    use bonsol_anchor_interface::{callback::handle_callback, instructions::InputRef};

    use super::*;

    pub fn initialize(_: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn welcome(ctx: Context<Welcome>, request_id: String, guess: u8) -> Result<()> {
        msg!("Hello, world!");
        let requester = ctx.accounts.initiator.key();
        let payer = ctx.accounts.initiator.key();
        let execution_account = &ctx.accounts.execution_account.key();
        let deployment_account = &ctx.accounts.deployment_account.key();
        let tip = 0;
        let expire = 0;
        let input_hash = keccak::hashv(&[&[guess]]);
        ctx.accounts.welcome_log.current_execution_account = Some(*execution_account);
        execute_v1_with_accounts(
            &requester,
            &payer,
            execution_account,
            deployment_account,
            WELCOME_IMAGE_ID,
            &request_id,
            vec![InputRef::public(&[guess])], // Input items for program
            tip,
            expire,
            ExecutionConfig {
                verify_input_hash: true,
                input_hash: Some(&input_hash.to_bytes()), // Hash of input items for program, keccak256
                forward_output: true,
            },
            Some(CallbackConfig {
                program_id: crate::id(),
                instruction_prefix: vec![0],
                extra_accounts: vec![],
            }),
            None,
        )
        .map_err(|_| WelcomeError::WelcomeRequestFailed)?;

        Ok(())
    }

    pub fn bonsol_callback(ctx: Context<BonsolCallback>, data: Vec<u8>) -> Result<()> {
        if let Some(execution_request_public_key) =
            ctx.accounts.welcome_log.current_execution_account
        {
            if ctx.accounts.execution_request.key() != execution_request_public_key {
                return Err(WelcomeError::InvalidCallback.into());
            }
            let account_infos = ctx.accounts.to_account_infos();
            let output = handle_callback(
                WELCOME_IMAGE_ID,
                &execution_request_public_key,
                &account_infos.as_slice(),
                &data,
            )
            .map_err(|_| WelcomeError::CallbackFailed)?;

            msg!("committted_outputs = {:?}", output.committed_outputs);
            Ok(())
        } else {
            Err(WelcomeError::InvalidCallback.into())
        }
    }
}

#[account]
#[derive(InitSpace)]
pub struct WelcomeLog {
    pub current_execution_account: Option<Pubkey>,
}

#[derive(Accounts)]
pub struct Welcome<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,

    #[account(
        init_if_needed,
        space = 8 + WelcomeLog::INIT_SPACE,
        payer = initiator,
        seeds = [b"welcomelog"],
        bump,
    )]
    pub welcome_log: Account<'info, WelcomeLog>,

    pub execution_account: Account<'info, ExecutionRequestV1Account<'info>>,
    pub deployment_account: Account<'info, DeployV1Account<'info>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct BonsolCallback<'info> {
    /// CHECK: This is the raw ER account, checked in the callback handler
    pub execution_request: UncheckedAccount<'info>,
    pub initiator: UncheckedAccount<'info>,

    #[account(mut, seeds = [b"welcomelog"], bump)]
    pub welcome_log: Account<'info, WelcomeLog>,
}
