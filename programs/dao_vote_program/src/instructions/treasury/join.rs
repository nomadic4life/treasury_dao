use crate::errors::ErrorCode;
use crate::states::{MemberTreasuryStatus, ProgramAuthority, TreasuryStatus, MEMBER_STATUS};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct CreateMemberTreasuryStatus<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        init,
        payer = member,
        space = MemberTreasuryStatus::LEN + 1000,
        seeds = [
            member.key().as_ref(),
            MEMBER_STATUS.as_bytes(),
        ],
        bump,
    )]
    pub member_status: Account<'info, MemberTreasuryStatus>,

    #[account(
        mut,
        address = program_authority.treasury_status
            @ ErrorCode::InvalidTreasuryStatus,
    )]
    pub treasury_status: AccountLoader<'info, TreasuryStatus>,

    #[account(mut)]
    pub member_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        address = program_authority.treasury_vault
            @ ErrorCode::InvalidTreasuryVault,
    )]
    pub treasury_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        address = program_authority.treasury_mint
            @ ErrorCode::InvalidTreasuryMint,
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,

    pub program_authority: Account<'info, ProgramAuthority>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateMemberTreasuryStatus<'info> {
    pub fn join(&mut self, amount: u64, bumps: &CreateMemberTreasuryStatusBumps) -> Result<()> {
        let treasury_status = &mut self.treasury_status.load_mut()?;

        self.member_status
            .init(bumps.member_status, self.member.key());

        treasury_status.deposit(amount)?;

        self.member_status.deposit(amount, &treasury_status);

        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.member_token_account.to_account_info(),
                    to: self.treasury_vault.to_account_info(),
                    authority: self.member.to_account_info(),
                    mint: self.token_mint.to_account_info(),
                },
            ),
            amount,
            self.token_mint.decimals,
        )?;

        Ok(())
    }
}
