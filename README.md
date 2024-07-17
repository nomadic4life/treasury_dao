# Multiplier Gainz Treasury DAO
## Description
It's a DAO that has control over a collective treasury, funds are put into the treasury and in exchange become a DAO member, you recieve tokens from Vault which are used to vote on proposals on which asset to swap with in the USDC treasury vault.  you can also stake your tokens to earn yield from taxes applied on the tokens that are used to make place your vote.

How it works.
1. Users can join the DAO by depositing into the treasury
2. The treasury is used to swap assets, and provide LP
3. The collective funds have the potential to magnifiy rewards
4. Members vote on which asset to swap, or provide luiquidity
5. Members can also earn in additional ways by locking up their DAO Tokens.
6. With all these investment opportunities members can maximize their gains.

With these features in mind gives the name Mulitpier Gainz Treasury DAO


## Current implemneted features
- Join DAO, become member
- Launch Phase claim DAO Token
- Withdraw funds from Treasury Vault
- Make Proposal
- Cast Vote on Proposal
- execute | swap asset on accpeted proposal
- stake tokens and earn from token vault
- claim | withdraw tokens from token vault

## Detailed overview of features and functionality
- Join DAO as Launching member
    - Deposit USDC into treasury vault, become a member.
    - After first funding round, launch members can claim their tokens.
    - Tokens are distributed based on the amount of tokens a user deposted in relation to the total deposited.
    - (user deposit / total deposit) * max launch token supply
    - Members can with draw their share of the treasury vault + asset vaults with only 10% max of their balance a week.
    - Once members treasury balance is 0, they are no longer members of the DAO.
    
- Vote Pipeline
    - A member can make a Proposal on which asset to swap using the treasury assets
    - Members can vote on the Proposal either yes or no by staking their tokens into the Ballot Vault.
    - Also Members can select a multiplier to be applied on their staked tokens to increase their voting power
    - After the deadline if yes > no.
    - Then the asset vault is created if not yet exist.
    - And then the Swap is made using the treasury asset for the asset that was voted on
    - To realize gains a proposal and vote must be made
    - if accepted, swap from the asset vault to the treasury vault (SELL),
    - Members can reclaim their tokens after the proposal deadline, a tax fee is applied
    - the tax fee rate is determined based on the mulitplier and the side of the vote.
    - higher multiper or losing side have higher applied tax fee rate

- Earn Tokens by Staking
    - Members Can lock up their tokens, in exchange to earn tokens from the DAO token vault
    - DAO tokens that were used to cast votes have a tax / penelity applied to them.
    - Those tokesn go into the DAO Token Vault, where the goal is to burn 1% a year.
    - And the rest can be earn by those who staked their tokens in the DAO Token Vault
    - The formula is still being worked out.

## Planned features
- Earn Yield on treasury assets that are used as LP and loans
    - members need to stake their DAO tokens into the yield vault
    - and members can earn a share of that yield

- Earn Gains on treasury assets being sold for profit
    - members need to stake their DAO tokens into the gains vault
    - and members can earn a share of the gains

- Frontend for users to interface with.
- Add addtional AMM's to have access to a wide range of investment instruments.

## Getting Started
### Prerequisits
- Rust
- Anchor
- Solana CLI

### Installation

1. Clone the repository:

```
git clone https://github.com/nomadic4life/treasury_dao.git
cd treasury_dao
```

2. run local test:

```
anchor test
```