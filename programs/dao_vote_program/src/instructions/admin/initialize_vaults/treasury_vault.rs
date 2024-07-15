use crate::states::{ProgramAuthority, TREASURY_VAULT_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct InitializeTreasuryVault<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [
            program_authority.key().as_ref(),
            TREASURY_VAULT_SEED.as_bytes(),
        ],
        bump,
        token::authority = program_authority,
        token::mint = treasury_mint,
        token::token_program = treasury_program,
    )]
    pub treasury_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        address = program_authority.treasury_mint,
        // ErrorCode::InvalidTreasuryMint
    )]
    pub treasury_mint: InterfaceAccount<'info, Mint>,

    pub treasury_program: Interface<'info, TokenInterface>,
    pub program_authority: Account<'info, ProgramAuthority>,
    pub system_program: Program<'info, System>,
}
