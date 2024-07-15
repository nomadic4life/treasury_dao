use crate::states::{
    AllocationTracker, StatusType, TreasuryStatus, AUTHORITY_SEED, TREASURY_STATUS_SEED,
};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct TransferRentZeroCopyTreasury<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = AllocationTracker::LEN,
        seeds = [
            TREASURY_STATUS_SEED.as_bytes(),
        ],
        bump
    )]
    pub allocation_tracker: Account<'info, AllocationTracker>,

    #[account(
        mut,
        seeds = [
            program_authority.key().as_ref(),
            TREASURY_STATUS_SEED.as_bytes(),
        ],
        bump,
    )]
    pub token_status: SystemAccount<'info>,

    #[account(
        seeds = [
            AUTHORITY_SEED.as_bytes(),
        ],
        bump,
    )]
    pub program_authority: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> TransferRentZeroCopyTreasury<'info> {
    pub fn transfer_rent(&mut self) -> Result<()> {
        self.allocation_tracker.init(
            StatusType::TreasuryStatus,
            self.token_status.key(),
            self.program_authority.key(),
        );

        let space = TreasuryStatus::LEN;
        let rent = Rent::get()?.minimum_balance(space.try_into().expect("overflow"));

        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.payer.to_account_info(),
                    to: self.token_status.to_account_info(),
                },
            ),
            rent,
        )?;

        Ok(())
    }
}
