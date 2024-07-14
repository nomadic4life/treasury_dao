use crate::states::{ProgramAuthority, ProposalConfig, TokenStatus, TreasuryStatus};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeAuthority<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = ProgramAuthority::LEN + 100,
        seeds = [
            b"authority"
        ],
        bump,
    )]
    pub program_authority: Box<Account<'info, ProgramAuthority>>,

    #[account(
        init,
        payer = payer,
        space = ProposalConfig::LEN + 100,
        seeds = [
            b"proposal-config"
        ],
        bump,
    )]
    pub proposal_config: Box<Account<'info, ProposalConfig>>,

    #[account(
        seeds = [
            program_authority.key().as_ref(),
            b"treasury-status"
        ],
        bump,
    )]
    pub treasury_status: AccountLoader<'info, TreasuryStatus>,

    #[account(
        seeds = [
            program_authority.key().as_ref(),
            b"token-status"
        ],
        bump,
    )]
    pub token_status: AccountLoader<'info, TokenStatus>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeAuthority<'info> {
    pub fn init(&mut self, bump: u8) -> Result<()> {
        self.program_authority.bump = bump;
        self.program_authority.proposal_config = self.proposal_config.key();
        self.program_authority.token_status = self.token_status.key();
        self.program_authority.treasury_status = self.treasury_status.key();

        Ok(())
    }
}
