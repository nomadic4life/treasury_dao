use crate::states::ProgramAuthority;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct InitializeTokenVault<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"authority"
        ],
        bump,
    )]
    pub program_authority: Box<Account<'info, ProgramAuthority>>,

    #[account(
        init,
        payer = payer,
        seeds = [
            program_authority.key().as_ref(),
            b"token-vault"
        ],
        bump,
        token::authority = program_authority,
        token::mint = token_mint,
        token::token_program = token_program,
    )]
    pub token_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        seeds = [
            program_authority.key().as_ref(),
            b"dao-token-mint",
        ],
        bump,
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeTokenVault<'info> {
    pub fn init(&mut self) -> Result<()> {
        self.program_authority.token_vault = self.token_vault.key();

        Ok(())
    }
}
