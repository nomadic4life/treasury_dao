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
        seeds = [
            member.key().as_ref(),
            b"member-status"
        ],
        bump,
    )]
    pub member_status: Box<Account<'info, MemberTreasuryStatus>>,

    #[account(
        mut,
        address = program_authority.treasury_status,
    )]
    pub treasury_status: Box<Account<'info, TreasuryStatus>>,

    #[account(mut)]
    pub member_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        address = program_authority.treasury_vault,
    )]
    pub treasury_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        address = program_authority.treasury_token_mint,
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub program_authority: Box<Account<'info, ProgramAuthority>>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> TreasuryDeposit<'info> {
    pub fn deposit(&mut self) -> Result<()> {
        self.treasury_status.update()?;

        self.member_status.update(&self.treasury_status);

        Ok(())
    }
}
