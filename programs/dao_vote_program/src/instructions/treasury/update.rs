use crate::errors::ErrorCode;
use crate::states::{MemberTreasuryStatus, ProgramAuthority, TreasuryStatus};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct TreasuryDeposit<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        mut,
        constraint = member_status.authority == member.key(),
    )]
    pub member_status: Box<Account<'info, MemberTreasuryStatus>>,

    #[account(
        mut,
        address = program_authority.treasury_status,
    )]
    pub treasury_status: Box<Account<'info, TreasuryStatus>>,

    pub program_authority: Box<Account<'info, ProgramAuthority>>,
}

impl<'info> TreasuryDeposit<'info> {
    pub fn deposit(&mut self) -> Result<()> {
        self.treasury_status.update()?;

        self.member_status.update(&self.treasury_status);

        Ok(())
    }
}
