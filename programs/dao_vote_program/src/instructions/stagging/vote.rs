use crate::states::{MemberStatus, MemberVoteStatus, ProgramAuthority};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
struct Vote<'info> {
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
            proposal.key().as_ref(),
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
            proposal.key().as_ref(),
            "ballet-vault",
        ],
        bump = proposal.ballet_vault_bump,
    )]
    pub ballet_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub proposal: Account<'info, Proposal>,
    pub member_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Vote<'info> {
    pub fn cast(&mut self, amount: u64, multiplier: u8, secret_vote: Hash) -> Result<()> {
        self.member_vote_status
            .init(self.member.key(), amount, mulitplier, secret_vote);

        transfer_checked(
            CpiContext::new(
                token_program.to_account_info(),
                TransferChecked {
                    from: member_token_account.to_account_info(),
                    to: ballet_vault.to_account_info(),
                    authority: member.to_account_info(),
                    mint: token_mint.to_account_info(),
                },
            ),
            amount,
            token_mint.decimals,
        )?
    }
}

// member locks the amount of tokens to make a vote into the ballet_vault
// ballet state -> in favor | against -> yes | no
// votes that have 50% or greater win the outcome of the proposal
// unlocking the tokens from the ballet_vault inposes a tax
// votes with in 60% : 40% range have 5% : 5% tax
// votes within 70% : 30% range have 0% : 10% tax
// votes with in 80% : 20% range have 0% : 15% tax and winning side collects 1% of those taxes
// the taxed tokens enter into a redistribution vault, and 1% of those tokens get burned
// target burn rate is 5% a year
// members can use a multiplier to scale their vote, though the taxed imposed on them is higher
// 1 token fee is also applied to encourage to maximize their vote
// the vote by default will go through if it doesn't meet a threshold, to encourage more engagement from voting
// so if people really don't want the vote to go through, they must actively vote no
// that threshold is 2% of the token supply
