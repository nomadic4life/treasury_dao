use crate::errors::ErrorCode;
use crate::states::{MemberTokenStatus, MemberTreasuryStatus, ProgramAuthority, TokenStatus};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    // checking constraint doesn't work on optional accounts, need to manually check with require!
    pub member_treasury_status: Option<Account<'info, MemberTreasuryStatus>>,

    #[account(
        mut,
        constraint = member_status.authority == member.key()
            @ ErrorCode::InvalidMemberEarnTokenStatus,
    )]
    pub member_status: Account<'info, MemberTokenStatus>,

    #[account(mut)]
    pub member_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        address = program_authority.token_status
            @ ErrorCode::InvalidEarnTokenStatus,
    )]
    pub token_status: AccountLoader<'info, TokenStatus>,

    #[account(
        mut,
        address = program_authority.token_vault
            @ ErrorCode::InvalidVault,
    )]
    pub token_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        address = program_authority.token_mint
            @ ErrorCode::InvalidTokenMint,
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    pub program_authority: Box<Account<'info, ProgramAuthority>>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimTokens<'info> {
    pub fn claim(&mut self, amount: u64) -> Result<()> {
        let token_status = &mut self.token_status.load_mut()?;
        token_status.withdraw(amount)?;

        // right now is_treasury member_status false, since there is no way to inlcude yet
        self.member_status.withdraw(amount, &token_status, false);

        let seeds = &[b"authority", &[self.program_authority.bump][..]];
        let signer_seeds = &[&seeds[..]];

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.token_vault.to_account_info(),
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
