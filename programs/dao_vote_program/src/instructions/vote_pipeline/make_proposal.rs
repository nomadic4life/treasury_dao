use crate::states::{
    MemberTreasuryStatus, PositionProposal, ProgramAuthority, ProposalConfig,
    POSITION_PROPOSAL_SEED,
};
use anchor_lang::prelude::*;

use anchor_spl::token_interface::TokenAccount;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct CreatePositionProposal<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        constraint = member_status.authority == member.key(),
        // ErrorCode::InvalidTreasuryMember,
        constraint = member_status.is_valid_member(),
        // ErrorCode::InvalidTreasuryMember,
    )]
    pub member_status: Account<'info, MemberTreasuryStatus>,

    // in future it will be necessary to get the data from the pool state
    /// CHECKED: Just need to store the pool state pubkey.
    pub pool_state: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = input_asset_vault.owner == program_authority.key(),
        // ErrorCode::InvalidAssetVaultOwner
        constraint = input_asset_vault.amount >= amount
        // ErrorCode::AssetVaultInsufficientAmount
    )]
    pub input_asset_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = member,
        space = PositionProposal::LEN,
        seeds = [
            // should be number not string, 
            // but is difficult to get test right with serlizing numbers
            // proposal_config.index.to_be_bytes().as_ref(),
            proposal_config.index.to_string().as_bytes(),
            POSITION_PROPOSAL_SEED.as_bytes(),
        ],
        bump
    )]
    pub position_proposal: Account<'info, PositionProposal>,

    #[account(
        mut,
        address = program_authority.proposal_config,
        // ErrorCode::InvalidProposalConfig
    )]
    pub proposal_config: Account<'info, ProposalConfig>,

    pub program_authority: Account<'info, ProgramAuthority>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreatePositionProposal<'info> {
    pub fn make_proposal(
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
