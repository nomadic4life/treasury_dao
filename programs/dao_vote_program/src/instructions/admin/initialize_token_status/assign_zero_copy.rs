use crate::errors::ErrorCode;
use crate::states::{AllocationTracker, StatusType, TOKEN_STATUS_SEED};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{allocate, assign, Allocate, Assign};

#[derive(Accounts)]
pub struct AssignZeroCopyTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        constraint = allocation_tracker.status_type == StatusType::TokenStatus
            @ ErrorCode::InvalidAllocationTracker,
    )]
    pub allocation_tracker: Account<'info, AllocationTracker>,

    #[account(
        mut,
        seeds = [
            program_authority.key().as_ref(),
            TOKEN_STATUS_SEED.as_bytes(),
        ],
        bump,
    )]
    pub token_status: SystemAccount<'info>,

    #[account(
        address = allocation_tracker.program_authority
            @ ErrorCode::InvalidProgramAuthorityAccount,
    )]
    pub program_authority: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> AssignZeroCopyTokens<'info> {
    pub fn assign(&mut self, bumps: &AssignZeroCopyTokensBumps, program_id: Pubkey) -> Result<()> {
        let space = self.allocation_tracker.increase();
        let seeds = &[
            self.program_authority.key.as_ref(),
            TOKEN_STATUS_SEED.as_bytes(),
            &[bumps.token_status][..],
        ];
        let signer_seeds = &[&seeds[..]];

        allocate(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Allocate {
                    account_to_allocate: self.token_status.to_account_info(),
                },
                signer_seeds,
            ),
            space,
        )?;

        assign(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Assign {
                    account_to_assign: self.token_status.to_account_info(),
                },
                signer_seeds,
            ),
            &program_id,
        )?;

        Ok(())
    }
}
