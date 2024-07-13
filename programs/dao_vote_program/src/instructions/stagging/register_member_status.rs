pub struct RegisterMemberStatus<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        init,
        payer = member,
        space = MemberStatus::Len,
        seeds = [
            member.key().as_ref,
            "member-status"
        ],
        bump,
    )]
    pub created_member_status: Account<'info, MemberStatus>,

    pub system_program: Program<'info, System>,
}
