mod constants;
mod instructions;
mod states;
mod utils;
use anchor_lang::prelude::*;

use instructions::*;

declare_id!("9TyRZTroFNPpQNLs8q5zEZdEHGJXzTeXNtjYx9n9W5XA");

#[program]
pub mod dao_vote_program {

    use super::*;

    pub fn initialize_program(ctx: Context<InitializeProgram>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)
    }

    pub fn join_dao(ctx: Context<CreateMemberTreasuryStatus>, amount: u64) -> Result<()> {
        ctx.accounts.join(amount, &ctx.bumps)
    }

    pub fn launch(ctx: Context<LaunchToken>) -> Result<()> {
        ctx.accounts.claim()
    }

    pub fn treasury_deposit(ctx: Context<TreasuryDeposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn treasury_claim(ctx: Context<TreasuryClaim>) -> Result<()> {
        ctx.accounts.claim()
    }

    pub fn create_position_proposal(
        ctx: Context<CreatePositionProposal>,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.create_position_proposal(&ctx.bumps, amount)
    }

    pub fn cast_vote(
        ctx: Context<CastVote>,
        amount: u64,
        multiplier: u8,
        is_yes: bool,
    ) -> Result<()> {
        ctx.accounts.cast_vote(amount, multiplier, is_yes)
    }

    pub fn deposit_token_vault(ctx: Context<StakeTokenVault>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw_token_vault(ctx: Context<StakeTokenVault>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn update_token_vault(ctx: Context<StakeTokenVault>) -> Result<()> {
        ctx.accounts.update()
    }

    pub fn deposit_treasury_vault(ctx: Context<StakeTreasuryVault>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn testing_swap(ctx: Context<Swap>) -> Result<()> {
        ctx.accounts.swap()
    }
}

// ideation::

// state
// vaults
// escrows
// treasury
// interact with AMMs

// critera to vote
//  - most be a DAO member
//  -

// critera to be a DAO member

// need token for program
// token will be used to make votes
// weight of vote is based on the number of tokens submitted
// - token : vote -> 1:1, 2:2, 4:3, 8:4, 16:5, 32:6, etc...
// tokens used to vote will enter into an escrow
//  - a portion will be burnt
//  - a portion will go to those who staked the token to earn more
//  - a portion will be available for those who to claim, to become DAO members

// what will users be voting on?
//  vault for yes -> the amount hidden then later revealed after vote
//  vault for no -> the amount hidden then later revealed after vote
//  after voting process -> transfer vault <-> escrow

// submit a proposal
//  -> there is a tiny cost to submit? want to create as many opportunities to vote on purposals,
//  -> so the friction to submitting a proposal should be small
//  - sell | buy | mint LP | burn LP | lock yield | unlock yield -> asset
//  - amount
//  - trigger price
//  - trigger date

// DAO MEMBERS CRITERA
//  - must transfer USDC into the DAO treasury
//  - the ownership ratio is determine by the amount that is transfered into the DAO treasury
//  - launch process
//      -> the launch process is the initializing the treasury for the first time
//      -> it will take place for a set peroid of time so the tresury can fill
//      -> once the launch phase completes, the ownership ratio is determined
//      -> and the program tokens are distributed based accordingling to this ratio.
//  - on going process
//      -> the on going process will take place after the first vote
//      -> an user can deposit into the treasury anytime to become a DAO member
//      -> and the ratio will be determine by the amount the was deposit and the current state of the treasury + investments of that time
//      -> though the new DAO member will not receive tokens immedaitely,
//      -> the amount of tokens will be owed to them that will come from the vault aspect
//      -> that accumulates over time and the DAO member can claim their tokens as that vault fills
//      -> DAO member can also buy the tokens to vote as well.
