use crate::states::ProgramAuthority;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface};

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
    pub new_authority: Box<Account<'info, ProgramAuthority>>,

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

    #[account(
        init,
        payer = payer,
        seeds = [
            new_authority.key().as_ref(),
            b"launch-vault"
        ],
        bump,
        token::authority = new_authority,
        token::mint = new_token_mint,
        token::token_program = token_program,
    )]
    pub launch_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeProgram<'info> {
    pub fn init(&mut self, bump: u8) -> Result<()> {
        self.new_authority.bump = bump;
        Ok(())
    }

    pub fn mint(&self) -> Result<()> {
        let seeds = &[b"authority", &[self.new_authority.bump][..]];
        let signer_seeds = &[&seeds[..]];
        let max_supply = 100_000_000__000_000_000;

        mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.new_token_mint.to_account_info(),
                    to: self.launch_vault.to_account_info(),
                    authority: self.new_authority.to_account_info(),
                },
                signer_seeds,
            ),
            max_supply,
        )?;
        Ok(())
    }
}
