use crate::states::{AllocationTracker, TreasuryStatus};
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
            b"treasury-status",
        ],
        bump
    )]
    pub allocation_tracker: Account<'info, AllocationTracker>,

    #[account(
        mut,
        seeds = [
            program_authority.key().as_ref(),
            b"treasury-status"
        ],
        bump,
    )]
    pub token_status: SystemAccount<'info>,

    #[account(
        seeds = [
            b"authority"
        ],
        bump,
    )]
    pub program_authority: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> TransferRentZeroCopyTreasury<'info> {
    pub fn transfer_rent(&mut self) -> Result<()> {
        self.allocation_tracker.init(1);
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
