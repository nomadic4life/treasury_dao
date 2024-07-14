use crate::states::{MemberTokenStatus, ProgramAuthority, TokenStatus};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateTokenStatus<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    // how to optionally include the member treasury status?
    #[account(
        mut,
        constraint = member_status.authority == member.key(),
    )]
    pub member_status: Account<'info, MemberTokenStatus>,

    #[account(
        mut,
        address = program_authority.token_status,
    )]
    pub token_status: Box<Account<'info, TokenStatus>>,

    pub program_authority: Box<Account<'info, ProgramAuthority>>,
}

impl<'info> UpdateTokenStatus<'info> {
    pub fn update(&mut self) -> Result<()> {
        self.token_status.update()?;

        // right now is_treasury member_status false, since there is no way to inlcude yet
        self.member_status.update(&self.token_status, false);

        Ok(())
    }
}
