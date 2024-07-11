mod constants;
mod instructions;
mod states;
use anchor_lang::prelude::*;

use instructions::*;

declare_id!("9TyRZTroFNPpQNLs8q5zEZdEHGJXzTeXNtjYx9n9W5XA");

#[program]
pub mod dao_vote_program {

    use super::*;

    pub fn initialize_program(ctx: Context<InitializeProgram>) -> Result<()> {
        ctx.accounts.init()
    }
}

#[derive(Accounts)]
pub struct Initialize {}

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
