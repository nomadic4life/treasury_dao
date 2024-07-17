use crate::errors::ErrorCode;
use crate::states::{MemberTreasuryStatus, ProgramAuthority, TreasuryStatus};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct LaunchToken<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(mut)]
    pub program_authority: Account<'info, ProgramAuthority>,

    #[account(
        mut,
        constraint = member_status.authority == member.key()
            @ ErrorCode::InvalidTreasuryMember,
        constraint = member_status.is_valid_launch_member()
            @ ErrorCode::InvalidTreasuryMember,
    )]
    pub member_status: Account<'info, MemberTreasuryStatus>,

    #[account(
        mut,
        address = program_authority.treasury_status
            @ ErrorCode::InvalidTreasuryStatus,
        // constraint = treasury_status.load()?.is_valid_launch()
        //     @ ErrorCode::InvalidLaunch,
    )]
    pub treasury_status: AccountLoader<'info, TreasuryStatus>,

    #[account(mut)]
    pub member_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        address = program_authority.launch_vault
            @ ErrorCode::InvalidLaunchVault,
    )]
    pub launch_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        address = program_authority.token_mint
            @ ErrorCode::InvalidTokenMint,
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

// NOTE: needs validation that its ready to launch token
// but for testing we are leaving out that validation
impl<'info> LaunchToken<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let treasury_status = &mut self.treasury_status.load_mut()?;
        let seeds = &[b"authority", &[self.program_authority.bump][..]];
        let signer_seeds = &[&seeds[..]];

        let round = 0;
        let (deposit_total, _) = treasury_status.get_totals_of_round(round);

        // making some assumptions -> will come back to this to handle interger overflow
        let balance = self.member_status.deposit_total;
        let share = (balance * 100_00 / deposit_total) as u128;
        let amount = (self.program_authority.max_supply as u128 * share / 100_00) as u64;

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
