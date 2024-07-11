use crate::states::ProgramAuthority;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = ProgramAuthority::LEN,
        seeds = [
            b"authority"
        ],
        bump,
    )]
    pub new_authority: Account<'info, ProgramAuthority>,

    #[account(
        init,
        payer = payer,
        seeds = [
            new_authority.key().as_ref(),
            b"dao-token-mint",
        ],
        bump,
        mint::authority = new_authority,
        mint::decimals = 9,
        mint::freeze_authority = new_authority,
    )]
    pub new_token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeProgram<'info> {
    pub fn init(&mut self) -> Result<()> {
        // self.new_program_authority.Ok(())
        Ok(())
    }
}
