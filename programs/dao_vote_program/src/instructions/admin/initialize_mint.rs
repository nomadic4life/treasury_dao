use crate::errors::ErrorCode;
use crate::states::{ProgramAuthority, DAO_TOKEN_MINT_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

// mint tokens
#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub program_authority: Account<'info, ProgramAuthority>,

    #[account(
        init,
        payer = payer,
        seeds = [
            program_authority.key().as_ref(),
            DAO_TOKEN_MINT_SEED.as_bytes(),
        ],
        bump,
        mint::authority = program_authority,
        mint::decimals = 6,
        mint::freeze_authority = program_authority,
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        address = program_authority.token_program
            @ ErrorCode::InvalidTokenProgram
    )]
    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}
