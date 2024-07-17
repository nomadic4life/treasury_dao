import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DaoVoteProgram } from "../target/types/dao_vote_program";

import {
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  createAssociatedTokenAccountInstruction,
  createInitializeMintInstruction,
  getMinimumBalanceForRentExemptMint,
  createMintToInstruction,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  // ASSOCIATED_TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";

import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

import fs from 'fs/promises';
const TEST_USERS_KEYPAIR_PATH = './test-users/';

const {
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
  PublicKey,
  Keypair,
} = anchor.web3

class Token {
  // mint: anchor.web3.Keypair | anchor.web3.PublicKey;
  // mint: anchor.web3.Keypair;
  mint: anchor.web3.Keypair;
  mintAuthority: anchor.web3.Keypair;
  freezeAuthority: anchor.web3.Keypair;
  supply: number;
  decimals: number;
  isInitialized: boolean;


  createMint = async (
    connection: anchor.web3.Connection,
    payer: anchor.web3.Keypair,
  ) => {

    this.mint = anchor.web3.Keypair.generate()
    this.mintAuthority = anchor.web3.Keypair.generate()
    this.freezeAuthority = anchor.web3.Keypair.generate()
    this.supply = 0
    this.decimals = 9
    this.isInitialized = false

    const lamports = await getMinimumBalanceForRentExemptMint(connection);
    const blockhash = await connection.getLatestBlockhash()

    const transaction = new Transaction({ ...blockhash, feePayer: payer.publicKey }).add(

      SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: this.mint.publicKey,
        space: MINT_SIZE,
        lamports,
        programId: TOKEN_PROGRAM_ID,
      }),

      createInitializeMintInstruction(
        this.mint.publicKey,
        this.decimals,
        this.mintAuthority.publicKey,
        this.freezeAuthority.publicKey
      )
    )


    const tx = await sendAndConfirmTransaction(connection, transaction, [payer, this.mint])

    await connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, "confirmed")


    await this.mintToken({ connection, payer })
  }

  createTokenAccount = async ({ connection, payer }) => {

    const tokenAccount = await getAssociatedTokenAddress(
      this.mint.publicKey,
      // this.mint.keypair.publicKey,

      payer.publicKey,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    )

    const transaction = new Transaction().add(
      createAssociatedTokenAccountInstruction(
        payer.publicKey,
        tokenAccount,
        payer.publicKey,
        this.mint.publicKey,
        // this.mint.keypair.publicKey,

        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      ))


    const tx = await sendAndConfirmTransaction(connection, transaction, [payer])

    const blockhash = connection.getLatestBlockhash()
    await connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, "confirmed")

  }

  mintToken = async ({
    connection,
    payer,
  }) => {

    await this.createTokenAccount({ connection, payer, })

    const tokenAccount = await getAssociatedTokenAddress(
      this.mint.publicKey,
      payer.publicKey,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    )

    const ix = createMintToInstruction(
      this.mint.publicKey,
      tokenAccount,
      this.mintAuthority.publicKey,
      10000 * LAMPORTS_PER_SOL,
    )

    const blockhash = await connection.getLatestBlockhash()
    const transaction = new Transaction(blockhash)
    transaction.add(ix).sign(payer)

    const tx = await sendAndConfirmTransaction(connection, transaction, [payer, this.mintAuthority])

    await connection.confirmTransaction({
      ...blockhash,
      signature: tx
    }, "confirmed")

  }
}


describe("dao_vote_program", () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  let provider = anchor.getProvider();

  const program = anchor.workspace.DaoVoteProgram as Program<DaoVoteProgram>;
  let tokenMint = new Token;
  let payer = new anchor.web3.Keypair();


  before("CREATE FAWK USDC TOKEN MINT", async () => {

    const tx = await provider.connection.requestAirdrop(
      payer.publicKey,
      10000 * anchor.web3.LAMPORTS_PER_SOL
    )

    const latestBlockHash = await provider.connection.getLatestBlockhash()

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });

    await tokenMint.createMint(provider.connection, payer);
  })


  describe("Initialize Process", () => {

    describe("Zero Copy Accounts", () => {

      it("Transfer Rent Treasury", async () => {

        await program.methods
          .transferRentZeroCopyTreasury()
          .accounts({})
          .rpc();

        const [allocationTracker] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("treasury-status")],
          program.programId
        )

        // console.log("")
        // console.log(" :: allocator tracker -> treasury ::")
        // let data = await program.account.allocationTracker.fetch(allocationTracker)
        // console.log(data)

      })

      it("Transfer Rent Tokens", async () => {

        await program.methods
          .transferRentZeroCopyTokens()
          .accounts({})
          .rpc();

        const [allocationTracker] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("token-status")],
          program.programId
        )

        // console.log("")
        // console.log(" :: allocator tracker -> token ::")
        // let data = await program.account.allocationTracker.fetch(allocationTracker)
        // console.log(data)
      })

      it("Assign And Allocate Treasury Status", async () => {
        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [allocationTracker] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("treasury-status")],
          program.programId
        )

        await program.methods
          .assignZeroCopyTreasury()
          .accounts({
            // payer?
            allocationTracker,
            // treasuryStatus?
            programAuthority,
            // systemProgram?
          })
          .rpc();
      })

      it("Assign And Allocate Token Status", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [allocationTracker] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("token-status")],
          program.programId
        )


        await program.methods
          .assignZeroCopyToken()
          .accounts({
            // payer?
            allocationTracker,
            // tokenStatus?
            programAuthority,
            // systemProgram?
          })
          .rpc();
      })

      it("Realloc Treasury", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [treasuryStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("treasury-status")
          ],
          program.programId
        )

        const [allocationTracker] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("treasury-status")],
          program.programId
        )

        await program.methods
          .reallocZeroCopyTreasury()
          .accounts({
            // payer?
            allocationTracker,
            treasuryStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury()
          .accounts({
            // payer?
            allocationTracker,
            treasuryStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury()
          .accounts({
            // payer?
            allocationTracker,
            treasuryStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury()
          .accounts({
            // payer?
            allocationTracker,
            treasuryStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury()
          .accounts({
            // payer?
            allocationTracker,
            treasuryStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury()
          .accounts({
            // payer?
            allocationTracker,
            treasuryStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury()
          .accounts({
            // payer?
            allocationTracker,
            treasuryStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury()
          .accounts({
            // payer?
            allocationTracker,
            treasuryStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury()
          .accounts({
            // payer?
            allocationTracker,
            treasuryStatus,
            programAuthority,
          })
          .rpc();


      })

      it("Realloc Tokens", async () => {
        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [tokenStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("token-status")
          ],
          program.programId
        )

        const [allocationTracker] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("token-status")],
          program.programId
        )

        await program.methods
          .reallocZeroCopyTokens()
          .accounts({
            // payer?
            allocationTracker,
            tokenStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTokens()
          .accounts({
            // payer?
            allocationTracker,
            tokenStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTokens()
          .accounts({
            // payer?
            allocationTracker,
            tokenStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTokens()
          .accounts({
            // payer?
            allocationTracker,
            tokenStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTokens()
          .accounts({
            // payer?
            allocationTracker,
            tokenStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTokens()
          .accounts({
            // payer?
            allocationTracker,
            tokenStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTokens()
          .accounts({
            // payer?
            allocationTracker,
            tokenStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTokens()
          .accounts({
            // payer?
            allocationTracker,
            tokenStatus,
            programAuthority,
          })
          .rpc();

        await program.methods
          .reallocZeroCopyTokens()
          .accounts({
            // payer?
            allocationTracker,
            tokenStatus,
            programAuthority,
          })
          .rpc();

      })

      it("Initialize Treasury", async () => {

        await program.methods
          .initializeZeroCopyTreasury()
          .accounts({})
          .rpc();
      })

      it("Initialize Tokens", async () => {

        const tx = await program.methods
          .initializeZeroCopyTokens()
          .accounts({})
          .rpc();
      })
    })

    describe("Initialize Program", () => {

      it("Initialize Authority", async () => {

        await program.methods
          .initializeAuthority()
          .accounts({
            treasuryMint: tokenMint.mint.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .rpc();

      })

      it("Initialize DAO Mint", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        await program.methods
          .initializeMint()
          .accounts({
            // payer?
            programAuthority,
            // tokenMint?
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .rpc();

      })
    })

    describe("Initialize Vaults", () => {

      it("Initialize Ballot Vault", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("dao-token-mint")
          ],
          program.programId
        )

        await program.methods
          .initializeBallotVaults()
          .accounts({
            // payer?,
            // ballotVault?,
            tokenMint: programTokenMint,
            programAuthority,
            tokenProgram: TOKEN_PROGRAM_ID,
            // systemProgram?
          })
          .rpc();
      })

      it("Initialize Launch Vault", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("dao-token-mint")
          ],
          program.programId
        )

        await program.methods
          .initializeLaunchVaults()
          .accounts({
            // payer?,
            // launchVault?,
            tokenMint: programTokenMint,
            programAuthority,
            tokenProgram: TOKEN_PROGRAM_ID,
            // systemProgram?
          })
          .rpc();
      })

      it("Initialize Tokens Vault", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("dao-token-mint")
          ],
          program.programId
        )

        await program.methods
          .initializeTokenVaults()
          .accounts({
            // payer?,
            // tokenVault?,
            tokenMint: programTokenMint,
            programAuthority,
            tokenProgram: TOKEN_PROGRAM_ID,
            // systemProgram?
          })
          .rpc();
      })

      it("Initialize Treasury Vault", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        await program.methods
          .initializeTreasuryVaults()
          .accounts({
            // payer?,
            // treasuryVault?,
            programAuthority,
            treasuryMint: tokenMint.mint.publicKey,
            treasuryProgram: TOKEN_PROGRAM_ID,
            // systemProgram?
          })
          .rpc();
      })

      it("Mint Max Supply", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("dao-token-mint")
          ],
          program.programId
        )

        const [launchVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("launch-vault")
          ],
          program.programId
        )

        await program.methods
          .mintMaxSupply()
          .accounts({
            // payer?
            programAuthority,
            tokenMint: programTokenMint,
            launchVault,
            tokenProgram: TOKEN_PROGRAM_ID,
            // systemProgram?
          })
          .rpc();

      })
    })

  })

  describe("User Interaction", () => {

    describe("Launch Phase", () => {

      it("Join DAO -> Launch Members Deposit", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [treasuryVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("treasury-vault")
          ],
          program.programId
        )

        const [treasuryStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("treasury-status")
          ],
          program.programId
        )

        const memberTokenAccount = await getOrCreateAssociatedTokenAccount(
          provider.connection, // connection
          payer, // payer
          // programTokenMint, // mint
          tokenMint.mint.publicKey,
          payer.publicKey,// owner
        )

        const [memberStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            payer.publicKey.toBuffer(),
            Buffer.from("member-status")
          ],
          program.programId
        )



        await program.methods
          .joinDao(new anchor.BN(10 * 1_000_000))
          .accountsPartial({
            member: payer.publicKey,
            treasuryVault,
            treasuryStatus,
            memberStatus,
            memberTokenAccount: memberTokenAccount.address,
            tokenMint: tokenMint.mint.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            programAuthority,
            systemProgram: SYSTEM_PROGRAM_ID,
          })
          .signers([payer])
          .rpc();
      })

      it("Launch Token -> Launch Members Claim -> NEED TO ADVANCE THE SLOT A SIGNIFICANT AMOUNT TO FULLY TEST", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [treasuryStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("treasury-status")
          ],
          program.programId
        )

        const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("dao-token-mint")
          ],
          program.programId
        )

        const memberTokenAccount = await getOrCreateAssociatedTokenAccount(
          provider.connection,
          payer,
          programTokenMint,
          payer.publicKey,
        )

        const [memberStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            payer.publicKey.toBuffer(),
            Buffer.from("member-status")
          ],
          program.programId
        )

        const [launchVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("launch-vault")
          ],
          program.programId
        )

        await program.methods
          .launch()
          .accountsPartial({
            member: payer.publicKey,
            memberStatus,
            treasuryStatus,
            memberTokenAccount: memberTokenAccount.address,
            launchVault,
            tokenProgram: TOKEN_PROGRAM_ID,
            programAuthority,
            tokenMint: programTokenMint
          })
          .signers([payer])
          .rpc();
      })

    })

    describe("Treasury Member -> NEED TO ADVACE THE SLOT A SIGNIFICANT AMOUNT TO FULLY TEST FEATURES", () => {

      it("Join DAO Treasury Member -> Non-Launch Member", async () => { })

      it("Deposit Into Treasury", async () => { })

      it("Update Treasury Status", async () => { })

      it("Claim From Treasury", async () => { })

    })

    describe("Vote Pipeline", () => {

      it("Create Position Proposal", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [mockPoolStatePubkey] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("mock-pool-state")],
          program.programId
        )

        const [proposalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("proposal-config")],
          program.programId
        )

        const [positionProposal] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("0"),
            Buffer.from("position-proposal"),
          ],
          program.programId
        )

        const [treasuryVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("treasury-vault")
          ],
          program.programId
        )

        const [memberStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            payer.publicKey.toBuffer(),
            Buffer.from("member-status")
          ],
          program.programId
        )

        await program.methods.makeProposal(new anchor.BN(100))
          .accountsPartial({
            member: payer.publicKey,
            memberStatus,
            poolState: mockPoolStatePubkey,
            inputAssetVault: treasuryVault,
            positionProposal,
            proposalConfig,
            programAuthority,
            // systemProgram?
          })
          .signers([payer])
          .rpc()
      })

      it("Member Cast Vote", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [memberStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            payer.publicKey.toBuffer(),
            Buffer.from("member-status")
          ],
          program.programId
        )

        const [positionProposal] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("0"),
            Buffer.from("position-proposal"),
          ],
          program.programId
        )

        const [ballotVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("ballot-vault")
          ],
          program.programId
        )

        const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("dao-token-mint")
          ],
          program.programId
        )

        const memberTokenAccount = await getOrCreateAssociatedTokenAccount(
          provider.connection,
          payer,
          programTokenMint,
          payer.publicKey,
        )

        await program.methods.castVote(new anchor.BN(100 * 1_000_000), 2, true)
          .accounts({
            member: payer.publicKey,
            memberStatus,
            // memberVoteStatus?,
            tokenMint: programTokenMint,
            ballotVault,
            memberTokenAccount: memberTokenAccount.address,
            programAuthority,
            positionProposal,
            tokenProgram: TOKEN_PROGRAM_ID,
            // systemProgram?,
          })
          .signers([payer])
          .rpc()
      })

      it("Members Reclaim Vote Token", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [positionProposal] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("0"),
            Buffer.from("position-proposal"),
          ],
          program.programId
        )

        const [ballotVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("ballot-vault")
          ],
          program.programId
        )

        const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("dao-token-mint")
          ],
          program.programId
        )

        const [memberStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            payer.publicKey.toBuffer(),
            Buffer.from("member-status")
          ],
          program.programId
        )

        const [memberVoteStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            payer.publicKey.toBuffer(),
            positionProposal.toBuffer(),
            Buffer.from("member-vote-status")
          ],
          program.programId
        )

        const memberTokenAccount = await getOrCreateAssociatedTokenAccount(
          provider.connection,
          payer,
          programTokenMint,
          payer.publicKey,
        )

        await program.methods.claimVoteToken()
          .accountsPartial({
            member: payer.publicKey,
            memberStatus,
            memberVoteStatus,
            tokenMint: programTokenMint,
            ballotVault,
            memberTokenAccount: memberTokenAccount.address,
            positionProposal,
            programAuthority,
            tokenProgram: TOKEN_PROGRAM_ID,
            // systemProgram
          })
          .signers([payer])
          .rpc()
      })

      it("Create Asset Status & Indexer & Vault", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [assetIndexer] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("0"),
            Buffer.from("asset-indexer")
          ],
          program.programId
        )

        const [assetConfig] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("asset-config")
          ],
          program.programId
        )

        await program.methods.createAssetStatus()
          .accountsPartial({
            payer: payer.publicKey,
            // assetVault?
            // assetStatus?
            assetIndexer,
            assetConfig,
            programAuthority,
            assetMint: tokenMint.mint.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            // systemProgram?
          })
          .signers([payer])
          .rpc()

      })

      it("Execute Passed Position Proposal Swap -> USING MOCK POOL STATE & MOCK SWAP", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [positionProposal] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("0"),
            Buffer.from("position-proposal"),
          ],
          program.programId
        )

        const [mockAmmConfig] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("mock-amm-config"),
          ],
          program.programId
        )

        const [mockPoolState] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("mock-pool-state"),
          ],
          program.programId
        )

        const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("dao-token-mint")
          ],
          program.programId
        )

        const [treasuryVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("treasury-vault")
          ],
          program.programId
        )

        const [mockOutputAccount] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("token-vault")
          ],
          program.programId
        )

        const [mockInputVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("treasury-vault")
          ],
          program.programId
        )

        const [mockOutputVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("token-vault")
          ],
          program.programId
        )

        const [mockObservationState] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("mock-observation-state")
          ],
          program.programId
        )


        await program.methods.mockSwap()
          .accounts({
            payer: payer.publicKey,
            positionProposal,
            // authority?!
            // amm_config!
            ammConfig: mockAmmConfig,
            // pool_state!
            poolState: mockPoolState,
            // input_token_account
            inputTokenAccount: treasuryVault, // mock input account
            // output_token_account 
            outputTokenAccount: mockOutputAccount, // mock output acocunt
            // input_vault!
            inputVault: mockInputVault,
            // output_vault!
            outputVault: mockOutputVault,
            // input_token_program!
            inputTokenProgram: TOKEN_PROGRAM_ID,
            // output_token_program!
            outputTokenProgram: TOKEN_PROGRAM_ID,
            // input_token_mint!
            inputTokenMint: tokenMint.mint.publicKey,
            // output_token_mint!
            outputTokenMint: programTokenMint, // using as mock output
            observationState: mockObservationState,
            programAuthority,
          })
          .signers([payer])
          .rpc()
      })

    })

    describe("Earn System Locked Vote Tokens", () => {

      it("Intialize Status", async () => {

        await program.methods.initializeEarnTokenMemberStatus()
          .accounts({
            member: payer.publicKey
          })
          .signers([payer])
          .rpc()
      })

      it("Lock | Deposit Tokens Into Token Vault", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("dao-token-mint")
          ],
          program.programId
        )

        const [tokenVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("token-vault")
          ],
          program.programId
        )

        const [tokenStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("token-status")
          ],
          program.programId
        )

        const [memberStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            payer.publicKey.toBuffer(),
            Buffer.from("member-earn-token-status")
          ],
          program.programId
        )

        const memberTokenAccount = await getOrCreateAssociatedTokenAccount(
          provider.connection,
          payer,
          programTokenMint,
          payer.publicKey,
        )

        await program.methods.lockIntoTokenVault(new anchor.BN(100))
          .accounts({
            member: payer.publicKey,
            memberTreasuryStatus: null,
            memberStatus,
            memberTokenAccount: memberTokenAccount.address,
            tokenStatus,
            tokenVault,
            tokenMint: programTokenMint,
            programAuthority,
            tokenProgram: TOKEN_PROGRAM_ID,
            // systemProgram?
          })
          .signers([payer])
          .rpc()
      })

      it("Deposit | Add Tokens Into Token Vault -> NO UPDATE TAKING PLACE, NEED TO ADVANCE THE SLOT A SIGNIFIGANT AMOUNT TO TEST FULLY", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("dao-token-mint")
          ],
          program.programId
        )

        const [tokenVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("token-vault")
          ],
          program.programId
        )

        const [tokenStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("token-status")
          ],
          program.programId
        )

        const [memberStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            payer.publicKey.toBuffer(),
            Buffer.from("member-earn-token-status")
          ],
          program.programId
        )

        const memberTokenAccount = await getOrCreateAssociatedTokenAccount(
          provider.connection,
          payer,
          programTokenMint,
          payer.publicKey,
        )

        await program.methods.lockIntoTokenVault(new anchor.BN(100))
          .accounts({
            member: payer.publicKey,
            memberTreasuryStatus: null,
            memberStatus,
            memberTokenAccount: memberTokenAccount.address,
            tokenStatus,
            tokenVault,
            tokenMint: programTokenMint,
            programAuthority,
            tokenProgram: TOKEN_PROGRAM_ID,
            // systemProgram?
          })
          .signers([payer])
          .rpc()

      })

      it("Update Position -> NO UPDATE TAKING PLACE, NEED TO ADVANCE THE SLOT A SIGNIFIGANT AMOUNT TO TEST FULLY", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [memberStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            payer.publicKey.toBuffer(),
            Buffer.from("member-earn-token-status")
          ],
          program.programId
        )

        const [tokenStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("token-status")
          ],
          program.programId
        )

        await program.methods.updateTokenVault()
          .accounts({
            memberTreasuryStatus: null,
            member: payer.publicKey,
            memberStatus,
            tokenStatus,
            programAuthority,
          })
          .signers([payer])
          .rpc()
      })

      it("Claim Tokens", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [memberStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            payer.publicKey.toBuffer(),
            Buffer.from("member-earn-token-status")
          ],
          program.programId
        )

        const [tokenStatus] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("token-status")
          ],
          program.programId
        )

        const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("dao-token-mint")
          ],
          program.programId
        )

        const [tokenVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("token-vault")
          ],
          program.programId
        )

        const memberTokenAccount = await getOrCreateAssociatedTokenAccount(
          provider.connection,
          payer,
          programTokenMint,
          payer.publicKey,
        )

        await program.methods.withdrawFromTokenVault(new anchor.BN(10))
          .accounts({
            member: payer.publicKey,
            memberTreasuryStatus: null,
            memberStatus,
            memberTokenAccount: memberTokenAccount.address,
            tokenStatus,
            tokenVault,
            tokenMint: programTokenMint,
            programAuthority,
            tokenProgram: TOKEN_PROGRAM_ID,
            // systemProgram?
          })
          .signers([payer])
          .rpc()
      })

    })

    describe("Earn System Treasury Gains -> FEATURES NOT CURRENTLY IMPLEMENTED", () => {
      it("", async () => {
      })

      // it("Intialize Status", async () => {
      // })

      // it("Lock Tokens", async () => {
      // })

      // it("Deposit Tokens", async () => {
      // })

      // it("Update Position", async () => {
      // })

      // it("Claim Tokens", async () => {
      // })

    })

    describe("Earn System Yield Gains -> FEATURES NOT CURRENTLY IMPLEMENTED", () => {
      it("", async () => {
      })

      // it("Intialize Status", async () => {
      // })

      // it("Lock Tokens", async () => {
      // })

      // it("Deposit Tokens", async () => {
      // })

      // it("Update Position", async () => {
      // })

      // it("Claim Tokens", async () => {
      // })

    })

  })

})