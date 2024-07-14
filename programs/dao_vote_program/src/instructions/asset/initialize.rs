use crate::states::{AssetConfig, AssetIndexer, AssetVaultStatus, ProgramAuthority};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct CreateAssetStatus<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [
            token_mint.key().as_ref(),
            b"asset-vault"
        ],
        bump,
        token::authority = program_authority,
        token::mint = token_mint,
        token::token_program = token_program,
    )]
    pub asset_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = payer,
        space = AssetVaultStatus::LEN,
        seeds = [
            asset_vault.key().as_ref(),
            b"asset-status"
        ],
        bump,
    )]
    pub asset_status: Account<'info, AssetVaultStatus>,

    #[account(
        init,
        payer = payer,
        space = AssetIndexer::LEN,
        seeds = [
            // will change this to get the proper value
            // right now this allows for easy testing
            asset_config.next_index().to_string().as_bytes(),
            b"asset-indexer",
        ],
        bump,
        // currently this is taking place with creation of the asset status
        // though once I have the ability to remove assets, and use a queue,
        // to keep track of that information, then the init of asset_indexer
        // can take place in its own instruction
    )]
    pub asset_indexer: Account<'info, AssetIndexer>,

    #[account(mut)]
    pub asset_config: Account<'info, AssetConfig>,

    // should have validations from pool state -> but for now
    // this will work
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub program_authority: Box<Account<'info, ProgramAuthority>>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateAssetStatus<'info> {
    pub fn initialize(&mut self, bumps: &CreateAssetStatusBumps) -> Result<()> {
        let slot = Clock::get()?.slot;

        self.asset_indexer.init(
            bumps.asset_indexer,
            self.asset_config.next_index(),
            self.asset_status.key(),
        );

        self.asset_status.init(
            bumps.asset_status,
            self.token_mint.key(),
            self.asset_vault.key(),
            slot,
            self.asset_indexer.index,
        );

        self.asset_config.update_index();

        Ok(())
    }
}
