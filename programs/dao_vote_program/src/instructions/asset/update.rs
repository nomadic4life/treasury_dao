pub struct UpdateAssetValuation<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub asset_config: Account<'info, AssetConfig>,

    pub asset_status: Account<'info, AssetVaultStatus>,
    // okay the update process is going to be pretty complicated
    // will come back to this later
    // pub oracle:
}
