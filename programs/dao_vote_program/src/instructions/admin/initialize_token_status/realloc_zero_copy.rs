use crate::states::{AllocationTracker, StatusType};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ReallocZeroCopyTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        constraint = allocation_tracker.status_type == StatusType::TokenStatus,
        // ErrorCode::InvalidAllocationTracker
    )]
    pub allocation_tracker: Account<'info, AllocationTracker>,

    #[account(
        mut,
        address = allocation_tracker.target_account,
        // ErrorCode::InvalidTokenStatusAccount
    )]
    /// CHECKED: reacllocate token status
    pub token_status: AccountInfo<'info>,

    #[account(
        address = allocation_tracker.program_authority,
        // ErrorCode::InvalidProgramAuthorityAccount
    )]
    pub program_authority: SystemAccount<'info>,
}

impl<'info> ReallocZeroCopyTokens<'info> {
    pub fn realloc(&mut self) -> Result<()> {
        let space = self.allocation_tracker.increase();
        self.token_status.realloc(space as usize, false)?;
        // emit log -> total space allocated

        Ok(())
    }
}
