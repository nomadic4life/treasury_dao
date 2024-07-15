use crate::states::{
    // STATES
    ProgramAuthority,
    ProposalConfig,
    AUTHORITY_SEED,
    BALLOT_VAULT_SEED,
    DAO_TOKEN_MINT_SEED,
    LAUNCH_VAULT_SEED,
    PROPOSAL_CONFIG,
    TOKEN_STATUS_SEED,
    TOKEN_VAULT_SEED,
    TREASURY_STATUS_SEED,
    TREASURY_VAULT_SEED,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeAuthority<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = ProgramAuthority::LEN,
        seeds = [
           AUTHORITY_SEED.as_bytes(),
        ],
        bump,
    )]
    pub program_authority: Account<'info, ProgramAuthority>,

    #[account(
        init,
        payer = payer,
        space = ProposalConfig::LEN,
        seeds = [
            PROPOSAL_CONFIG.as_bytes(),
        ],
        bump,
    )]
    pub proposal_config: Account<'info, ProposalConfig>,

    #[account(
        seeds = [
            program_authority.key().as_ref(),
            TREASURY_STATUS_SEED.as_bytes(),
        ],
        bump,
    )]
    /// CHECKED: store the pubkey on to the program authority
    pub treasury_status: UncheckedAccount<'info>,

    #[account(
        seeds = [
            program_authority.key().as_ref(),
            TOKEN_STATUS_SEED.as_bytes(),
        ],
        bump,
    )]
    /// CHECKED: store the pubkey on to the program authority
    pub token_status: UncheckedAccount<'info>,

    #[account(
        seeds = [
            program_authority.key().as_ref(),
            BALLOT_VAULT_SEED.as_bytes(),
        ],
        bump,
    )]
    /// CHECKED: store the pubkey on to the program authority
    pub ballot_vault: UncheckedAccount<'info>,

    #[account(
        seeds = [
            program_authority.key().as_ref(),
            LAUNCH_VAULT_SEED.as_bytes(),
        ],
        bump,
    )]
    /// CHECKED: store the pubkey on to the program authority
    pub launch_vault: UncheckedAccount<'info>,

    #[account(
        seeds = [
            program_authority.key().as_ref(),
            TOKEN_VAULT_SEED.as_bytes(),
        ],
        bump,
    )]
    /// CHECKED: store the pubkey on to the program authority
    pub token_vault: UncheckedAccount<'info>,

    #[account(
        seeds = [
            program_authority.key().as_ref(),
            TREASURY_VAULT_SEED.as_bytes(),
        ],
        bump,
    )]
    /// CHECKED: store the pubkey on to the program authority
    pub treasury_vault: UncheckedAccount<'info>,

    #[account(
        seeds = [
            program_authority.key().as_ref(),
            DAO_TOKEN_MINT_SEED.as_bytes(),
        ],
        bump,
    )]
    /// CHECKED: store the pubkey on to the program authority
    pub token_mint: UncheckedAccount<'info>,

    /// CHECKED: store the pubkey on to the program authority
    pub treasury_mint: UncheckedAccount<'info>,

    /// CHECKED: store the pubkey on to the program authority
    pub token_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeAuthority<'info> {
    pub fn init(&mut self, bump: u8) -> Result<()> {
        self.program_authority.init(
            bump,
            self.treasury_vault.key(),
            self.ballot_vault.key(),
            self.launch_vault.key(),
            self.token_vault.key(),
            self.token_program.key(),
            self.treasury_mint.key(),
            self.token_mint.key(),
            self.treasury_status.key(),
            self.token_status.key(),
            self.proposal_config.key(),
        );

        Ok(())
    }
}
