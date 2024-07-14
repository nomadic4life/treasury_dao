use crate::states::MemberTokenStatus;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitliazeMemberTokenStatus<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        init,
        payer = member,
        space = MemberTokenStatus::LEN,
        seeds = [
            member.key().as_ref(),
            b"member-status"
        ],
        bump,
    )]
    pub member_status: Account<'info, MemberTokenStatus>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitliazeMemberTokenStatus<'info> {
    pub fn initialize(&mut self) -> Result<()> {
        self.member_status.init(self.member.key());

        Ok(())
    }
}
