use crate::constants::*;
use anchor_lang::prelude::*;
pub use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

pub const AUTHORITY_SEED: &str = "authority";
pub const DAO_TOKEN_MINT_SEED: &str = "dao-token-mint";
pub const BALLOT_VAULT_SEED: &str = "ballot-vault";
pub const LAUNCH_VAULT_SEED: &str = "launch-vault";
pub const TOKEN_VAULT_SEED: &str = "token-vault";
pub const TREASURY_VAULT_SEED: &str = "treasury-vault";

#[account]
pub struct ProgramAuthority {
    pub bump: u8,
    pub ballot_vault: Pubkey,
    pub treasury_vault: Pubkey,
    pub launch_vault: Pubkey,
    pub token_vault: Pubkey,

    pub token_program: Pubkey,

    pub treasury_mint: Pubkey,
    pub token_mint: Pubkey,

    pub treasury_status: Pubkey,
    pub token_status: Pubkey,

    pub proposal_config: Pubkey,
    pub asset_config: Pubkey,

    pub max_supply: u64,
}

impl ProgramAuthority {
    pub const LEN: usize = DISCRIMINATOR + BYTE + (PUBKEY_BYTES * 11) + UNSIGNED_64;
    pub const MAX_SUPPLY: u64 = 100_000_000_000__000_000;

    pub fn init(
        &mut self,
        bump: u8,

        treasury_vault: Pubkey,
        ballot_vault: Pubkey,
        launch_vault: Pubkey,
        token_vault: Pubkey,

        token_program: Pubkey,

        treasury_mint: Pubkey,
        token_mint: Pubkey,

        treasury_status: Pubkey,
        token_status: Pubkey,

        proposal_config: Pubkey,
        asset_config: Pubkey,
    ) {
        self.bump = bump;

        self.treasury_vault = treasury_vault;
        self.ballot_vault = ballot_vault;
        self.launch_vault = launch_vault;
        self.token_vault = token_vault;

        self.token_program = token_program;

        self.treasury_mint = treasury_mint;
        self.token_mint = token_mint;

        self.treasury_status = treasury_status;
        self.token_status = token_status;

        self.proposal_config = proposal_config;
        self.asset_config = asset_config;
    }
}
