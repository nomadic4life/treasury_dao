use crate::states::ProgramAuthority;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface};

// mint tokens
#[derive(Accounts)]
pub struct MintMaxSupply<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub program_authority: Account<'info, ProgramAuthority>,

    #[account(
        mut,
        address = program_authority.token_mint,
        // ErrorCode::InvalidTokenMint,        
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        address = program_authority.launch_vault,
        // ErrorCode::InvalidVault
    )]
    pub launch_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        address = program_authority.token_program,
        // ErrorCode::InvalidTokenProgram
    )]
    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

// NOTE: need to add constraint so can only mint max supply once
impl<'info> MintMaxSupply<'info> {
    pub fn mint(&mut self) -> Result<()> {
        self.program_authority.max_supply = ProgramAuthority::MAX_SUPPLY;

        let seeds = &[b"authority", &[self.program_authority.bump][..]];
        let signer_seeds = &[&seeds[..]];

        mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.token_mint.to_account_info(),
                    to: self.launch_vault.to_account_info(),
                    authority: self.program_authority.to_account_info(),
                },
                signer_seeds,
            ),
            ProgramAuthority::MAX_SUPPLY,
        )?;

        Ok(())
    }
}
