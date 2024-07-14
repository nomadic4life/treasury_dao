use anchor_lang::prelude::*;

use crate::states::{MemberTreasuryStatus, ProgramAuthority, TreasuryStatus};
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct LaunchToken<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        mut,
        constraint = member_status.authority == member.key(),
        constraint = member_status.is_valid_launch_member(),
    )]
    pub member_status: Box<Account<'info, MemberTreasuryStatus>>,

    #[account(
        address = program_authority.treasury_status,
        constraint = treasury_status.load()?.is_valid_launch(),
    )]
    pub treasury_status: AccountLoader<'info, TreasuryStatus>,

    #[account(mut)]
    pub member_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        address = program_authority.launch_vault,
    )]
    pub launch_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub program_authority: Box<Account<'info, ProgramAuthority>>,
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> LaunchToken<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let treasury_status = &mut self.treasury_status.load_mut()?;
        let seeds = &[b"authority", &[self.program_authority.bump][..]];
        let signer_seeds = &[&seeds[..]];

        let round = 0;
        let (valuation, _) = treasury_status.get_valuation_of_round(round);
        let balance = self.member_status.deposit_total;

        // could cause error if interger overflow for now this will work
        let share = balance * 100_00 / valuation;

        // could cause error if interger overflow for now this will work
        let amount = self.program_authority.max_supply * share / 100_00;

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.launch_vault.to_account_info(),
                    to: self.member_token_account.to_account_info(),
                    authority: self.program_authority.to_account_info(),
                    mint: self.token_mint.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
            self.token_mint.decimals,
        )?;

        self.member_status.claim_launch_status();

        Ok(())
    }
}
