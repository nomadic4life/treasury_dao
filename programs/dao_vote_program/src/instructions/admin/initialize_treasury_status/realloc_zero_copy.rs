use crate::states::AllocationTracker;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ReallocZeroCopyTreasury<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            program_authority.key().as_ref(),
            b"treasury-status"
        ],
        bump,
    )]
    /// CHECKED: reacllocate treasury status
    pub treasury_status: AccountInfo<'info>,

    #[account(
        seeds = [
            b"authority"
        ],
        bump,
    )]
    pub program_authority: SystemAccount<'info>,

    pub allocation_tracker: Account<'info, AllocationTracker>,
}

impl<'info> ReallocZeroCopyTreasury<'info> {
    pub fn realloc(&mut self) -> Result<()> {
        let space = self.allocation_tracker.increase();
        self.treasury_status.realloc(space as usize, false)?;

        Ok(())
    }
}
