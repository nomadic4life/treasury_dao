use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Allocation Tracker Status")]
    InvalidAllocationTracker,

    #[msg("Invalid Program Authority")]
    InvalidProgramAuthorityAccount,

    #[msg("Invalid Treasury Status Account")]
    InvalidTreasuryStatusAccount,

    #[msg("Invalid Token Status Account")]
    InvalidTokenStatusAccount,

    #[msg("Invalid Token Mint")]
    InvalidTokenMint,

    #[msg("Invalid Treasury Mint")]
    InvalidTreasuryMint,

    #[msg("Invalid Token Program")]
    InvalidTokenProgram,

    #[msg("Invalid Vault")]
    InvalidVault,

    // Invalid Asset Config?
    #[msg("Invalid Config")]
    InvalidConfig,

    #[msg("Invalid Proposal Config")]
    InvalidProposalConfig,

    #[msg("Invalid Member Earn Token Status")]
    InvalidMemberEarnTokenStatus,

    #[msg("Invalid Earn Token Status")]
    InvalidEarnTokenStatus,

    #[msg("Invalid Treasury Member")]
    InvalidTreasuryMember,

    #[msg("Invalid Treasury Status")]
    InvalidTreasuryStatus,

    #[msg("Invalid Launch")]
    InvalidLaunch,

    #[msg("Invalid Launch Vault")]
    InvalidLaunchVault,

    #[msg("Invalid Treasury Vault")]
    InvalidTreasuryVault,

    #[msg("Invalid Asset Vault Owner")]
    InvalidAssetVaultOwner,

    #[msg("Asset Vault Insufficient Funds")]
    AssetVaultInsufficientAmount,
}
