use crate::states::AllocationTracker;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ReallocZeroCopyTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [
            b"token-status",
        ],
        bump
    )]
    // this is not updating state???? WHY????
    pub allocation_tracker: Account<'info, AllocationTracker>,

    #[account(
        mut,
        seeds = [
            program_authority.key().as_ref(),
            b"token-status"
        ],
        bump,
    )]
    /// CHECKED: reacllocate token status
    pub token_status: AccountInfo<'info>,

    #[account(
        seeds = [
            b"authority"
        ],
        bump,
    )]
    pub program_authority: SystemAccount<'info>,
}

impl<'info> ReallocZeroCopyTokens<'info> {
    pub fn realloc(&mut self, space: u32) -> Result<()> {
        // let space = self.allocation_tracker.increase();
        self.token_status.realloc(space as usize, false)?;

        Ok(())
    }
}
