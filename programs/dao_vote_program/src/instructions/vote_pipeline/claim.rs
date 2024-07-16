use crate::errors::ErrorCode;
use crate::states::{
    MemberTreasuryStatus, MemberVoteStatus, PositionProposal, ProgramAuthority, MEMBER_VOTE_STATUS,
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct ClaimVotedTokens<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        constraint = member_status.authority == member.key()
            @ ErrorCode::InvalidTreasuryMember,
        constraint = member_status.is_valid_member()
            @ ErrorCode::InvalidTreasuryMember,
    )]
    pub member_status: Account<'info, MemberTreasuryStatus>,

    #[account(
        seeds = [
            member.key().as_ref(),
            position_proposal.key().as_ref(),
            MEMBER_VOTE_STATUS.as_bytes(),
        ],
        bump = member_vote_status.bump,
        // will change to this constraint
        // constraint = member_vote_status.position_proposal == position_proposal.key(),
        // ErrorCode::VoteOnInvalidePositionProposal
    )]
    pub member_vote_status: Account<'info, MemberVoteStatus>,

    #[account(
        mut,
        address = program_authority.token_mint
            @ ErrorCode::InvalidTokenMint,
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        address = program_authority.ballot_vault
            @ ErrorCode::InvalidVault,
    )]
    pub ballot_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub member_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        // NOTE: currently commented out for allow easy testing
        // constraint = position_proposal.is_valid_claim().unwrap(),
        // ErrorCode::VoteOnPositionProposalCurrentlyOpen
    )]
    pub position_proposal: Account<'info, PositionProposal>,

    pub program_authority: Box<Account<'info, ProgramAuthority>>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimVotedTokens<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let seeds = &[b"authority", &[self.program_authority.bump][..]];
        let signer_seeds = &[&seeds[..]];

        let amount = self.member_vote_status.amount;

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.ballot_vault.to_account_info(),
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
