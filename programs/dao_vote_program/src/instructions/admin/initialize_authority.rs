use crate::states::{ProgramAuthority, ProposalConfig};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeAuthority<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = ProgramAuthority::LEN,
        seeds = [
            b"authority"
        ],
        bump,
    )]
    pub program_authority: Box<Account<'info, ProgramAuthority>>,

    #[account(
        init,
        payer = payer,
        space = ProposalConfig::LEN,
        seeds = [
            b"proposal-config"
        ],
        bump,
    )]
    pub proposal_config: Box<Account<'info, ProposalConfig>>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeAuthority<'info> {
    pub fn init(&mut self, bump: u8) -> Result<()> {
        self.program_authority.bump = bump;
        self.program_authority.proposal_config = self.proposal_config.key();
        Ok(())
    }
}
