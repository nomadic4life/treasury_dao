use crate::states::{TreasuryStatus, AUTHORITY_SEED, TREASURY_STATUS_SEED};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeZeroCopyTreasury<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        zero,
        seeds = [
            program_authority.key().as_ref(),
           TREASURY_STATUS_SEED.as_bytes()
        ],
        bump,
    )]
    pub treasury_status: AccountLoader<'info, TreasuryStatus>,

    #[account(
        seeds = [
           AUTHORITY_SEED.as_bytes(),
        ],
        bump,
    )]
    pub program_authority: SystemAccount<'info>,
}

impl<'info> InitializeZeroCopyTreasury<'info> {
    pub fn init(&mut self) -> Result<()> {
        self.treasury_status.load_init()?;
        // emit log

        Ok(())
    }
}
