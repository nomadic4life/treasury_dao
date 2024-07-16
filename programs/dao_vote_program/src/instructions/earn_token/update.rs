use crate::states::{MemberTokenStatus, MemberTreasuryStatus, ProgramAuthority, TokenStatus};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateTokenStatus<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    // checking constraint doesn't work on optional accounts, need to manually check with require!
    pub member_treasury_status: Option<Account<'info, MemberTreasuryStatus>>,

    #[account(
        mut,
        constraint = member_status.authority == member.key(),
        // ErrorCode::InvalidMemberEarnTokenStatus
    )]
    pub member_status: Account<'info, MemberTokenStatus>,

    #[account(
        mut,
        address = program_authority.token_status,
        // ErrorCode::InvalidEarnTokenStatus
    )]
    pub token_status: AccountLoader<'info, TokenStatus>,

    pub program_authority: Box<Account<'info, ProgramAuthority>>,
}

impl<'info> UpdateTokenStatus<'info> {
    pub fn update(&mut self) -> Result<()> {
        let token_status = &mut self.token_status.load_mut()?;
        token_status.update()?;

        // right now is_treasury member_status false, since there is no way to inlcude yet
        self.member_status.update(&token_status, false);

        Ok(())
    }
}
