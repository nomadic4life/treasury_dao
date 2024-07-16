use crate::errors::ErrorCode;
use crate::states::{AllocationTracker, StatusType};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ReallocZeroCopyTreasury<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        constraint = allocation_tracker.status_type == StatusType::TreasuryStatus
            @ ErrorCode::InvalidAllocationTracker,
    )]
    pub allocation_tracker: Account<'info, AllocationTracker>,

    #[account(
        mut,
        address = allocation_tracker.target_account
            @ ErrorCode::InvalidTreasuryStatusAccount,
    )]
    /// CHECKED: reacllocate treasury status
    pub treasury_status: AccountInfo<'info>,

    #[account(
        address = allocation_tracker.program_authority
            @ ErrorCode::InvalidProgramAuthorityAccount,

    )]
    pub program_authority: SystemAccount<'info>,
}

impl<'info> ReallocZeroCopyTreasury<'info> {
    pub fn realloc(&mut self) -> Result<()> {
        let space = self.allocation_tracker.increase();
        self.treasury_status.realloc(space as usize, false)?;
        // emit log -> total space allocated

        Ok(())
    }
}
