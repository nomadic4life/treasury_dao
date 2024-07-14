use anchor_lang::prelude::*;
use anchor_lang::system_program::{allocate, assign, Allocate, Assign};

#[derive(Accounts)]
pub struct AssignZeroCopyTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

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

impl<'info> AssignZeroCopyTokens<'info> {
    const MAX_SPACE: u64 = 10240;

    pub fn assign(&mut self, bumps: &AssignZeroCopyTokensBumps, program_id: Pubkey) -> Result<()> {
        let seeds = &[
            self.program_authority.key.as_ref(),
            b"token-status",
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
            AssignZeroCopyTokens::MAX_SPACE,
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
