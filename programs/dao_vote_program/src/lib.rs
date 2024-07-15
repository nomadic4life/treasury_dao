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

    pub fn transfer_rent_zero_copy_treasury(
        ctx: Context<TransferRentZeroCopyTreasury>,
    ) -> Result<()> {
        ctx.accounts.transfer_rent()
    }

    pub fn transfer_rent_zero_copy_tokens(ctx: Context<TransferRentZeroCopyTokens>) -> Result<()> {
        ctx.accounts.transfer_rent()
    }

    pub fn assign_zero_copy_treasury(ctx: Context<AssignZeroCopyTreasury>) -> Result<()> {
        ctx.accounts.assign(&ctx.bumps, ctx.program_id.key())
    }

    pub fn assign_zero_copy_tokens(ctx: Context<AssignZeroCopyTokens>) -> Result<()> {
        ctx.accounts.assign(&ctx.bumps, ctx.program_id.key())
    }

    pub fn realloc_zero_copy_treasury(ctx: Context<ReallocZeroCopyTreasury>) -> Result<()> {
        ctx.accounts.realloc()
    }

    pub fn realloc_zero_copy_tokens(ctx: Context<ReallocZeroCopyTokens>) -> Result<()> {
        ctx.accounts.realloc()
    }

    pub fn initialize_zero_copy_treasury(ctx: Context<InitializeZeroCopyTreasury>) -> Result<()> {
        ctx.accounts.init()
    }

    pub fn initialize_zero_copy_tokens(ctx: Context<InitializeZeroCopyTokens>) -> Result<()> {
        ctx.accounts.init()
    }

    pub fn initialize_treasury_vaults(ctx: Context<InitializeTreasuryVault>) -> Result<()> {
        ctx.accounts.init()
    }

    pub fn initialize_launch_vaults(ctx: Context<InitializeLaunchVault>) -> Result<()> {
        ctx.accounts.init()
    }

    pub fn initialize_ballot_vaults(ctx: Context<InitializeBallotVault>) -> Result<()> {
        ctx.accounts.init()
    }

    pub fn initialize_token_vaults(ctx: Context<InitializeTokenVault>) -> Result<()> {
        ctx.accounts.init()
    }

    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        ctx.accounts.init()
    }

    pub fn mint_tokens(ctx: Context<MintTokens>) -> Result<()> {
        ctx.accounts.init()
    }

    pub fn initialize_authority(ctx: Context<InitializeAuthority>) -> Result<()> {
        ctx.accounts.init(ctx.bumps.program_authority)
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

    pub fn make_proposal(ctx: Context<CreatePositionProposal>, amount: u64) -> Result<()> {
        ctx.accounts.make_proposal(&ctx.bumps, amount)
    }

    pub fn cast_vote(
        ctx: Context<CastVote>,
        amount: u64,
        multiplier: u8,
        is_yes: bool,
    ) -> Result<()> {
        ctx.accounts
            .cast_vote(ctx.bumps.member_vote_status, amount, multiplier, is_yes)
    }

    pub fn claim_vote_token(ctx: Context<ClaimVotedTokens>) -> Result<()> {
        ctx.accounts.claim()
    }

    pub fn initialize_token_member_status(ctx: Context<InitliazeMemberTokenStatus>) -> Result<()> {
        ctx.accounts.initialize()
    }

    pub fn deposit_token_vault(ctx: Context<LockTokens>, amount: u64) -> Result<()> {
        ctx.accounts.lock(amount)
    }

    pub fn withdraw_token_vault(ctx: Context<ClaimTokens>, amount: u64) -> Result<()> {
        ctx.accounts.claim(amount)
    }

    pub fn update_token_vault(ctx: Context<UpdateTokenStatus>) -> Result<()> {
        ctx.accounts.update()
    }

    pub fn create_asset_status(ctx: Context<CreateAssetStatus>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
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

// TODO! need add
//  vaulation source
//  zero copy -> treasury status, token status
//  errors
//  event logs
//  improve the algo
//  test cases
