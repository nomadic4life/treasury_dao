use crate::states::{MemberTreasuryStatus, ProgramAuthority, TreasuryStatus};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct TreasuryClaim<'info> {
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
        // need to check if already claimed of the current round
    )]
    pub treasury_status: AccountLoader<'info, TreasuryStatus>,

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

impl<'info> TreasuryClaim<'info> {
    // need to claim
    pub fn claim(&mut self) -> Result<()> {
        let treasury_status = &mut self.treasury_status.load_mut()?;
        let amount = self.member_status.claim(&treasury_status);
        treasury_status.claim(amount)?;

        let seeds = &[b"authority", &[self.program_authority.bump][..]];
        let signer_seeds = &[&seeds[..]];

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.treasury_vault.to_account_info(),
                    to: self.member_token_account.to_account_info(),
                    authority: self.program_authority.to_account_info(),
                    mint: self.token_mint.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
            self.token_mint.decimals,
        )?;

        Ok(())
    }
}
