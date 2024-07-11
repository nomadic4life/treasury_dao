use crate::states::ProgramAuthority;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        has_one = authority,
    )]
    pub program_authority: Box<Account<'info, ProgramAuthority>>,

    #[account(
        mut,
        seeds = [
            program_authority.key().clone().as_ref(),
            b"dao-token-mint",
        ],
        bump
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        seeds = [
            b"vault"
        ],
        bump
    )]
    // vault | escrow?
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> MintTokens<'info> {
    pub fn mint(&self) -> Result<()> {
        let seeds = &[b"authority", &[self.program_authority.bump][..]];
        let signer_seeds = &[&seeds[..]];
        let amount = 100_000_000__000_000_000;

        mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.token_mint.to_account_info(),
                    to: self.vault.to_account_info(),
                    authority: self.program_authority.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;
        Ok(())
    }
}
