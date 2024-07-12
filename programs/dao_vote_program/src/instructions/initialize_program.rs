use crate::states::{ProgramAuthority, TokenVaultStatus, TreasuryStatus};
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
    pub new_token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub usdc_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = payer,
        seeds = [
            // program_authority.key().as_ref(),
            b"launch-vault"
        ],
        bump,
        token::authority = program_authority,
        token::mint = new_token_mint,
        token::token_program = token_program,
    )]
    pub launch_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = payer,
        seeds = [
            program_authority.key().as_ref(),
            b"treasury-vault"
        ],
        bump,
        token::authority = program_authority,
        token::mint = usdc_token_mint,
        token::token_program = usdc_token_program,
    )]
    pub treasury_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = payer,
        // in the future will use zero account abstraction
        // this will do for easy testing
        // space = TreasuryStatus::LEN,
        space = 10240,
        seeds = [
            // program_authority.key().as_ref(),
            b"treasury-status"
        ],
        bump,
    )]
    pub treasury_status: Box<Account<'info, TreasuryStatus>>,

    #[account(
        init,
        payer = payer,
        seeds = [
            program_authority.key().as_ref(),
            b"token-vault"
        ],
        bump,
        token::authority = program_authority,
        token::mint = new_token_mint,
        token::token_program = token_program,
    )]
    pub token_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = payer,
        // in the future will use zero account abstraction
        // this will do for easy testing
        // space = TreasuryStatus::LEN,
        space = 10240,
        seeds = [
            program_authority.key().as_ref(),
            b"tokne-vault-status"
        ],
        bump,
    )]
    pub token_vault_status: Box<Account<'info, TokenVaultStatus>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub usdc_token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeProgram<'info> {
    const MAX_SUPPLY: u64 = 10_000_000_000__000_000_000;

    pub fn init(&mut self, bumps: &InitializeProgramBumps) -> Result<()> {
        self.program_authority.bump = bumps.program_authority;
        self.program_authority.launch_vault_bump = bumps.launch_vault;
        self.program_authority.treasury_vault_bump = bumps.treasury_vault;
        self.program_authority.treasury_status_bump = bumps.treasury_status;
        self.program_authority.token_mint_bump = bumps.new_token_mint;
        self.program_authority.token_vault_bump = bumps.token_vault;
        self.program_authority.token_vault_status_bump = bumps.token_vault_status;

        self.program_authority.token_mint = self.new_token_mint.key();
        self.program_authority.max_supply = InitializeProgram::MAX_SUPPLY;

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
                    mint: self.new_token_mint.to_account_info(),
                    to: self.launch_vault.to_account_info(),
                    authority: self.program_authority.to_account_info(),
                },
                signer_seeds,
            ),
            InitializeProgram::MAX_SUPPLY,
        )?;
        Ok(())
    }
}
