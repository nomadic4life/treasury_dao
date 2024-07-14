use crate::states::TokenStatus;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeZeroCopyTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        zero,
        seeds = [
            program_authority.key().as_ref(),
            b"token-status"
        ],
        bump,
    )]
    pub token_status: AccountLoader<'info, TokenStatus>,

    #[account(
        seeds = [
            b"authority"
        ],
        bump,
    )]
    pub program_authority: SystemAccount<'info>,
}

impl<'info> InitializeZeroCopyTokens<'info> {
    pub fn init(&mut self) -> Result<()> {
        self.token_status.load_init()?;

        Ok(())
    }
}
