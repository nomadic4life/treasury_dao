use crate::states::{MemberTokenVaultStatus, ProgramAuthority, TokenVaultStatus};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct StakeTokenVault<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(mut)]
    pub member_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        seeds = [
            b"authority"
        ],
        bump = program_authority.bump,
    )]
    pub program_authority: Box<Account<'info, ProgramAuthority>>,

    #[account(
        mut,
        seeds = [
            program_authority.key().as_ref(),
            b"dao-token-mint",
        ],
        bump = program_authority.token_mint_bump,
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        seeds = [
            program_authority.key().as_ref(),
            b"token-vault",
        ],
        bump = program_authority.token_vault_bump,
    )]
    pub token_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        // can use authority instead of deriving from seeds
        seeds = [
            member.key().as_ref(),
            b"member-token-vault-status"
        ],
        bump
    )]
    pub member_token_vault_status: Box<Account<'info, MemberTokenVaultStatus>>,

    #[account(
        mut,
        // can use authority instead of deriving from seeds
        seeds = [
            program_authority.key().as_ref(),
            b"token-vault-status"
        ],
        bump
    )]
    pub token_vault_status: Box<Account<'info, TokenVaultStatus>>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> StakeTokenVault<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.member_token_account.to_account_info(),
                    to: self.token_vault.to_account_info(),
                    authority: self.member.to_account_info(),
                    mint: self.token_mint.to_account_info(),
                },
            ),
            amount,
            self.token_mint.decimals,
        )?;

        self.token_vault_status.deposit(amount)?;
        self.member_token_vault_status
            .deposit(amount, &self.token_vault_status);

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
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

        self.token_vault_status.withdraw(amount)?;
        self.member_token_vault_status
            .withdraw(amount, &self.token_vault_status);

        Ok(())
    }

    // should move this to seperate validation since don't need to transfer
    // but it works as is.
    pub fn update(&mut self) -> Result<()> {
        self.token_vault_status.update()?;
        self.member_token_vault_status
            .update(&self.token_vault_status);

        Ok(())
    }
}
