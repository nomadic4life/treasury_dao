use crate::states::{
    // MemberTreasuryStatus,
    PositionProposal,
    ProposalConfig,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreatePositionProposal<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    // #[account(
    //     constraint = member_status.authority == member.key(),
    //     constraint = member_status.last_round.is_some(),
    // )]
    // pub member_status: Box<Account<'info, MemberTreasuryStatus>>,
    /// CHECKED: Just need to store the pool state pubkey.
    pub pool_state: UncheckedAccount<'info>,

    #[account(
        init,
        payer = member,
        space = PositionProposal::LEN,
        seeds = [
            // should be number not string, 
            // but is difficult to get test right with serlizing numbers
            // proposal_config.index.to_be_bytes().as_ref(),
            proposal_config.index.to_string().as_bytes(),
            b"position-proposal",
        ],
        bump
    )]
    pub position_proposal: Box<Account<'info, PositionProposal>>,

    pub proposal_config: Box<Account<'info, ProposalConfig>>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreatePositionProposal<'info> {
    pub fn create_position_proposal(
        &mut self,
        bumps: &CreatePositionProposalBumps,
        amount: u64,
    ) -> Result<()> {
        self.position_proposal
            .init(bumps.position_proposal, self.pool_state.key(), amount)?;

        self.proposal_config.next();

        // emit event to signal new propsal was made start the voting process

        Ok(())
    }
}
