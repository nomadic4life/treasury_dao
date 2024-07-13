pub struct CreateProposalPosition<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        constraint member_status.authority = member.key(),
        constraint = member_status.last_round.is_some(),
    )]
    pub member_status: Box<Account<'info, MemberTreasuryStatus>>,

    #[account(
        constraint = program_authority.proposal_config == proposal_config.key(),
    )]
    pub proposal_config: Box<Account<'info, ProposalConfig>>,

    #[account(
        init,
        payer = member,
        space = position_proposal::LEN,
        seeds = [
            proposal_config.index,
            b"position-proposal",
        ],
        bump
    )]
    pub position_proposal: Box<Account<'info, PositionProposal>>,

    #[account(
        constraint input_token_account.key() != output_token_account.key(),

        constraint = input_token_account.mint == pool_state.load()?.token_0_mint 
        || input_token_account.mint == pool_state.load()?.token_1_mint,

        constraint = input_token_account.mint == pool_state.load()?.token_0_mint 
        || input_token_account.mint == pool_state.load()?.token_1_mint 
    )]
    pub pool_state: AccountLoader<'info, PoolState>,

    /// CHECKED: Just need pubkey
    pub pool_state: UncheckedAccount<'info>,

    #[account(
        constraint = asset_token_mint.key() == output_token_account.mint,
    )]
    pub asset_token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub program_authority: Box<Account<'info, ProgramAuthority>>,
    pub input_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub output_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateProposalPosition<'info> {
    pub fn create(
        &mut self, 
        bumps: &InitializeProgramBumps, 
        price: u64, 
        amount: u64, 
        action: Action
    ) {
        // bounty value for test purpose
        let bounty = 10__000_000_000;

        self.proposal_position.init(
            bumps.position_proposal,
            self.pool_state.key(),
            self.input_token_account.key(),
            self.output_token_account.key(),
            // bounty,
            // price,
            amount,
            // action,
        );

        self.proposal_config.next();
    }
}
