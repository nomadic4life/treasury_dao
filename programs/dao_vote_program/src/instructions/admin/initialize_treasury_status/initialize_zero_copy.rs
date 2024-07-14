use crate::states::TreasuryStatus;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeZeroCopyTreasury<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        zero,
        seeds = [
            program_authority.key().as_ref(),
            b"treasury-status"
        ],
        bump,
    )]
    pub treasury_status: AccountLoader<'info, TreasuryStatus>,

    #[account(
        seeds = [
            b"authority"
        ],
        bump,
    )]
    pub program_authority: SystemAccount<'info>,
}

impl<'info> InitializeZeroCopyTreasury<'info> {
    pub fn init(&mut self) -> Result<()> {
        self.treasury_status.load_init()?;
        Ok(())
    }
}
