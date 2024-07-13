use crate::states::{
    // MemberStatus,
    MemberVoteStatus,
    PositionProposal,
    ProgramAuthority,
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        seeds = [
            b"authority"
        ],
        bump = program_authority.bump,
    )]
    pub program_authority: Box<Account<'info, ProgramAuthority>>,

    // #[account(
    //     constraint = member_status.authority == member.key(),
    //     constraint = member_status.is_active(),
    // )]
    // pub member_status: Account<'info, MemberStatus>,
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
            b"ballot-vault",
        ],
        bump = program_authority.ballot_vault_bump,
    )]
    pub ballot_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub member_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub position_proposal: Account<'info, PositionProposal>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CastVote<'info> {
    // need validations to limit the multiplier so not to have interger overflow issues
    pub fn cast_vote(&mut self, amount: u64, multiplier: u8, is_yes: bool) -> Result<()> {
        self.member_vote_status
            .init(self.member.key(), amount, multiplier);

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
