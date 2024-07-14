use crate::states::ProgramAuthority;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [
            b"authority"
        ],
        bump,
    )]
    pub program_authority: Account<'info, ProgramAuthority>,

    #[account(
        mut,
        seeds = [
            program_authority.key().as_ref(),
            b"dao-token-mint",
        ],
        bump,
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = payer,
        seeds = [
            program_authority.key().as_ref(),
            b"launch-vault"
        ],
        bump,
        token::authority = program_authority,
        token::mint = token_mint,
        token::token_program = token_program,
    )]
    pub launch_vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintTokens<'info> {
    const MAX_SUPPLY: u64 = 10_000_000_000__000_000_000;

    pub fn init(&mut self) -> Result<()> {
        msg!("TOKEN VAULT::: {}", self.launch_vault.key());
        self.program_authority.launch_vault = self.launch_vault.key();
        self.program_authority.token_mint = self.token_mint.key();
        self.program_authority.max_supply = MintTokens::MAX_SUPPLY;

        msg!("TOKEN VAULT::: {}", self.program_authority.launch_vault);
        self.mint()?;

        Ok(())
    }

    pub fn mint(&self) -> Result<()> {
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
            MintTokens::MAX_SUPPLY,
        )?;
        Ok(())
    }
}
