use crate::states::{AllocationTracker, TokenStatus};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
#[instruction(seed: bool)]
pub struct TransferRentZeroCopyTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = AllocationTracker::LEN,
        seeds = [
            AllocationTracker::get(seed).as_bytes(),
        ],
        bump
    )]
    pub allocation_tracker: Account<'info, AllocationTracker>,

    #[account(
        mut,
        seeds = [
            program_authority.key().as_ref(),
            b"token-status"
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

impl<'info> TransferRentZeroCopyTokens<'info> {
    pub fn transfer_rent(&mut self, seed: bool) -> Result<()> {
        self.allocation_tracker.init(seed);

        let space = TokenStatus::LEN;

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
