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

  it("Is initialized!", async () => {
    // need usdc token mint

    console.log(tokenMint.mint)

    const tx = await program.methods
      .initializeProgram()
      .accounts({
        usdcTokenMint: tokenMint.mint.publicKey,
        // tokenProgram: TOKEN_2022_PROGRAM_ID,

        // for testing, because can't figure out how to create ATA with TOKEN_2022_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,

        usdcTokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    // .prepare();
    console.log("Your transaction signature", tx);
  });

  it("Create Position Proposal!", async () => {

    const [mockPoolStatePubkey] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("mock-pubkey")],
      program.programId
    )

    const [proposalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("proposal-config")],
      program.programId
    )

    const tx = await program.methods
      .createPositionProposal(new anchor.BN(1_000_000_000))
      .accounts({
        // member
        // memeber treasury status
        // pool state 
        // position proposal
        // proposal config
        // sytem program
        member: payer.publicKey,
        poolState: mockPoolStatePubkey,
        proposalConfig,
      })
      .signers([payer])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  // how this test is currently set up. it allows me to test the voting process 
  // without worring about token issuance from launch vault
  it("transfer tokens!", async () => {
    const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId
    )

    const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        programAuthority.toBuffer(),
        Buffer.from("dao-token-mint"),
      ],
      program.programId
    )

    const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        programAuthority.toBuffer(),
        Buffer.from("launch-vault"),
      ],
      program.programId
    )

    const receipentATA = await getOrCreateAssociatedTokenAccount(
      provider.connection, // connection
      payer, // payer
      programTokenMint, // mint
      payer.publicKey,// owner
    )

    const tx = await program.methods
      .transferTokens()
      .accounts({
        programAuthority,
        vault: vault,
        receipent: receipentATA.address,
        tokenProgram: TOKEN_PROGRAM_ID,
        // tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();

    console.log("Your transaction signature", tx);

  });
});
