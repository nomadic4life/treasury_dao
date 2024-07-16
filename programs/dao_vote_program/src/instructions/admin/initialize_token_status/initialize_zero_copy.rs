use crate::states::{TokenStatus, AUTHORITY_SEED, TOKEN_STATUS_SEED};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeZeroCopyTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        zero,
        seeds = [
            program_authority.key().as_ref(),
            TOKEN_STATUS_SEED.as_bytes()
        ],
        bump,
    )]
    pub token_status: AccountLoader<'info, TokenStatus>,

    #[account(
        seeds = [
            AUTHORITY_SEED.as_bytes(),
        ],
        bump,
    )]
    pub program_authority: SystemAccount<'info>,
}

impl<'info> InitializeZeroCopyTokens<'info> {
    pub fn init(&mut self) -> Result<()> {
        self.token_status.load_init()?;
        // emit log

        Ok(())
    }
}
