use crate::states::{MemberTreasuryStatus, MemberVoteStatus, PositionProposal, ProgramAuthority};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        constraint = member_status.authority == member.key(),
        constraint = member_status.is_valid_member(),
    )]
    pub member_status: Account<'info, MemberTreasuryStatus>,

    #[account(
        init,
        payer = member,
        space = MemberVoteStatus::LEN,
        seeds = [
            member.key().as_ref(),
            position_proposal.key().as_ref(),
            b"member-vote-status"
        ],
        bump,
    )]
    pub member_vote_status: Account<'info, MemberVoteStatus>,

    #[account(
        mut,
        address = program_authority.token_mint,
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        address = program_authority.ballot_vault,
    )]
    pub ballot_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub member_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub program_authority: Box<Account<'info, ProgramAuthority>>,
    pub position_proposal: Account<'info, PositionProposal>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CastVote<'info> {
    // need validations to limit the multiplier so not to have interger overflow issues
    pub fn cast_vote(&mut self, bump: u8, amount: u64, multiplier: u8, is_yes: bool) -> Result<()> {
        self.member_vote_status
            .init(bump, self.member.key(), amount, multiplier);

        // self.position_proposal.vote(is_yes, amount << mulitplier);

        // for now for easy testing, and not worry about interger overflow issues
        self.position_proposal
            .vote(is_yes, amount * multiplier as u64);

        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.member_token_account.to_account_info(),
                    to: self.ballot_vault.to_account_info(),
                    authority: self.member.to_account_info(),
                    mint: self.token_mint.to_account_info(),
                },
            ),
            amount,
            self.token_mint.decimals,
        )?;

        Ok(())
    }
}
