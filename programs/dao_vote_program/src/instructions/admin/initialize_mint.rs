use crate::states::ProgramAuthority;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
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
            b"dao-token-mint",
        ],
        bump,
        mint::authority = program_authority,
        mint::decimals = 9,
        mint::freeze_authority = program_authority,
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

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
    pub launch_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeMint<'info> {
    const MAX_SUPPLY: u64 = 10_000_000_000__000_000_000;

    pub fn init(&mut self) -> Result<()> {
        self.program_authority.launch_vault = self.launch_vault.key();
        self.program_authority.token_mint = self.token_mint.key();
        self.program_authority.max_supply = InitializeMint::MAX_SUPPLY;

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
            InitializeMint::MAX_SUPPLY,
        )?;
        Ok(())
    }
}
