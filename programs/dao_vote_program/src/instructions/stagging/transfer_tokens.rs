use anchor_lang::prelude::*;

use crate::states::ProgramAuthority;

use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    // needs multi sig functionality
    #[account(mut)]
    pub signer: Signer<'info>,

    pub program_authority: Account<'info, ProgramAuthority>,

    #[account(mut)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub receipent: InterfaceAccount<'info, TokenAccount>,

    // if this going to serve all vaults, then needs a better check
    #[account(
        seeds = [
            program_authority.key().clone().as_ref(),
            b"dao-token-mint",
        ],
        bump = program_authority.token_mint_bump,
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> TransferTokens<'info> {
    pub fn transfer(&mut self) -> Result<()> {
        let seeds = &[b"authority", &[self.program_authority.bump][..]];
        let signer_seeds = &[&seeds[..]];

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.vault.to_account_info(),
                    to: self.receipent.to_account_info(),
                    authority: self.program_authority.to_account_info(),
                    mint: self.token_mint.to_account_info(),
                },
                signer_seeds,
            ),
            1_000__000_000_000,
            self.token_mint.decimals,
        )?;

        Ok(())
    }
}
