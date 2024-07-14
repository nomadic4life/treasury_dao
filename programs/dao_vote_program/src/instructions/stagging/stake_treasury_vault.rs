use crate::states::{MemberTokenVaultStatus, ProgramAuthority, TokenVaultStatus};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct StakeTreasuryVault<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(mut)]
    pub member_usdc_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            program_authority.key().as_ref(),
            b"treasury-vault",
        ],
        bump = program_authority.treasury_vault_bump,
    )]
    pub treasury_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        seeds = [
            b"authority"
        ],
        bump = program_authority.bump,
    )]
    pub program_authority: Box<Account<'info, ProgramAuthority>>,

    #[account(
        mut,
        // can use authority instead of deriving from seeds
        seeds = [
            member.key().as_ref(),
            b"member-treasury-status"
        ],
        bump
    )]
    pub member_treasury_status: Box<Account<'info, MemberTokenVaultStatus>>,

    #[account(
        mut,
        // can use authority instead of deriving from seeds
        seeds = [
            program_authority.key().as_ref(),
            b"treasury-status"
        ],
        bump
    )]
    pub treasury_status: Box<Account<'info, TokenVaultStatus>>,

    pub usdc_mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> StakeTreasuryVault<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.member_usdc_account.to_account_info(),
                    to: self.treasury_vault.to_account_info(),
                    authority: self.member.to_account_info(),
                    mint: self.usdc_mint.to_account_info(),
                },
            ),
            amount,
            self.usdc_mint.decimals,
        )?;

        self.treasury_status.deposit(amount)?;
        self.member_treasury_status
            .deposit(amount, &self.treasury_status);

        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        self.treasury_status.update()?;
        self.member_treasury_status.update(&self.treasury_status);

        Ok(())
    }
    // claim -> not here?
    // withdraw -> not here and complicated? maybe go with simple version, just with draw usdc from vault
}
