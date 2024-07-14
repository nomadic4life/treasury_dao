use crate::states::ProgramAuthority;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct InitializeTreasuryVault<'info> {
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
            b"treasury-vault"
        ],
        bump,
        token::authority = program_authority,
        token::mint = treasury_token_mint,
        token::token_program = treasury_token_program,
    )]
    pub treasury_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub treasury_token_mint: Box<InterfaceAccount<'info, Mint>>,
    pub treasury_token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeTreasuryVault<'info> {
    pub fn init(&mut self) -> Result<()> {
        self.program_authority.treasury_vault = self.treasury_vault.key();

        Ok(())
    }
}
