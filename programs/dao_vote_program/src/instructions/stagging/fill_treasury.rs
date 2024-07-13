// send usdc tokens to treasury
// track the total balance of treasury
// track balance of member

// ratio -> member_balance.amount / treasury_balance.amount

struct FillTreasury<'info> {
    pub member: Signer<'info>,

    #[account(
        mut,
        constraint = authority = member.key(),
    )]
    pub member_treasury_status: Account<'info, MemberStatus>,

    #[account(
        mut,
        seeds = [
            b"treasury-status"
        ],
        bump = program_authority.bump,
    )]
    pub treasury_status: Account<'info, TreasuryStatus>,

    #[account(
        seeds = [
            b"authority"
        ],
        bump = program_authority.bump,
    )]
    pub program_authority: Account<'info, ProgramAuthority>,

    #[account(
        mut,
        seeds = [
            new_authority.key().as_ref(),
            b"treasury-vault"
        ],
        bump = program_authority.treasury_vault_bump,
    )]
    pub treasury_vault: InterfaceAccount<'info, TokenAccount>,

    pub member_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}
