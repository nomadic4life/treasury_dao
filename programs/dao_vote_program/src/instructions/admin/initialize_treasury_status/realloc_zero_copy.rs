use crate::states::AllocationTracker;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ReallocZeroCopyTreasury<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [
            b"treasury-status",
        ],
        bump
    )]
    // this is not updating state???? WHY????
    pub allocation_tracker: Account<'info, AllocationTracker>,

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
}

impl<'info> ReallocZeroCopyTreasury<'info> {
    pub fn realloc(&mut self, space: u32) -> Result<()> {
        // let space = self.allocation_tracker.increase();
        self.treasury_status.realloc(space as usize, false)?;

        Ok(())
    }
}
