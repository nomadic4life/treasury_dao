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
    // await tokenMint.mintToken({ connection: provider.connection, payer })
  })

  describe("Initialize Program", () => {

    describe("Zero Copy Accounts", () => {
      it("Transfer Rent Treasury", async () => {

        const tx = await program.methods
          .transferRentZeroCopyTreasury()
          .accounts({})
          .rpc();
      })

      it("Transfer Rent Tokens", async () => {

        const tx = await program.methods
          .transferRentZeroCopyTokens()
          .accounts({})
          .rpc();
      })

      it("Assign And Allocate Treasury", async () => {

        const tx = await program.methods
          .assignZeroCopyTreasury()
          .accounts({})
          .rpc();
      })

      it("Assign And Allocate Tokens", async () => {

        const tx = await program.methods
          .assignZeroCopyTokens()
          .accounts({})
          .rpc();
      })

      it("Realloc Treasury", async () => {

        await program.methods
          .reallocZeroCopyTreasury(10240 * 2)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury(10240 * 3)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury(10240 * 4)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury(10240 * 5)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury(10240 * 6)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury(10240 * 7)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury(10240 * 8)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury(10240 * 9)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTreasury(10240 * 10)
          .accounts({})
          .rpc();
      })

      it("Realloc Tokens", async () => {

        await program.methods
          .reallocZeroCopyTokens(10240 * 2)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTokens(10240 * 3)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTokens(10240 * 4)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTokens(10240 * 5)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTokens(10240 * 6)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTokens(10240 * 7)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTokens(10240 * 8)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTokens(10240 * 9)
          .accounts({})
          .rpc();

        await program.methods
          .reallocZeroCopyTokens(10240 * 10)
          .accounts({})
          .rpc();
      })

      it("Initialize Treasury", async () => {

        const tx = await program.methods
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

        const tx = await program.methods
          .initializeAuthority()
          .accounts({})
          .rpc();
      })

      it("Initialize Mint", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [launchVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("launch-vault")
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



        await program.methods
          .initializeMint()
          .accountsStrict({
            payer: payer.publicKey,
            programAuthority,
            tokenMint: programTokenMint,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SYSTEM_PROGRAM_ID,
          })
          .signers([payer])
          .rpc();

      })

      it("Mint Tokens", async () => {

        const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("authority")],
          program.programId
        )

        const [launchVault] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            programAuthority.toBuffer(),
            Buffer.from("launch-vault")
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

        await program.methods
          .mintTokens()
          .accountsStrict({
            payer: payer.publicKey,
            programAuthority,
            launchVault,
            tokenMint: programTokenMint,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SYSTEM_PROGRAM_ID,
          })
          .signers([payer])
          .rpc();

      })
    })

    describe("Initialize Vaults", () => {
      it("Initialize Ballot Vault", async () => {

        const tx = await program.methods
          .initializeBallotVaults()
          .accounts({
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .rpc();
      })

      it("Initialize Tokens Vault", async () => {

        const tx = await program.methods
          .initializeTokenVaults()
          .accounts({
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .rpc();
      })

      it("Initialize Treasury Vault", async () => {

        const tx = await program.methods
          .initializeTreasuryVaults()
          .accounts({
            treasuryTokenMint: tokenMint.mint.publicKey,
            treasuryTokenProgram: TOKEN_PROGRAM_ID,
          })
          .rpc();
      })
    })



  })

  describe("Launch Phase", () => {
    it("Join DAO", async () => {

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

      // const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
      //   [
      //     programAuthority.toBuffer(),
      //     Buffer.from("dao-token-mint")
      //   ],
      //   program.programId
      // )

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
        .joinDao(new anchor.BN(1 * 1_000_000))
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


    // it("Launch Token -> Launch Members Claim", async () => {

    //   const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
    //     [Buffer.from("authority")],
    //     program.programId
    //   )



    //   const [treasuryStatus] = anchor.web3.PublicKey.findProgramAddressSync(
    //     [
    //       programAuthority.toBuffer(),
    //       Buffer.from("treasury-status")
    //     ],
    //     program.programId
    //   )

    //   const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
    //     [
    //       programAuthority.toBuffer(),
    //       Buffer.from("dao-token-mint")
    //     ],
    //     program.programId
    //   )

    //   const memberTokenAccount = await getOrCreateAssociatedTokenAccount(
    //     provider.connection, // connection
    //     payer, // payer
    //     // programTokenMint, // mint
    //     tokenMint.mint.publicKey,
    //     payer.publicKey,// owner
    //   )

    //   const [memberStatus] = anchor.web3.PublicKey.findProgramAddressSync(
    //     [
    //       payer.publicKey.toBuffer(),
    //       Buffer.from("member-status")
    //     ],
    //     program.programId
    //   )

    //   const [launchVault] = anchor.web3.PublicKey.findProgramAddressSync(
    //     [
    //       programAuthority.toBuffer(),
    //       Buffer.from("launch-vault")
    //     ],
    //     program.programId
    //   )

    //   console.log(launchVault)

    //   const tx = await program.methods
    //     .launch()
    //     .accountsPartial({
    //       member: payer.publicKey,
    //       memberStatus,
    //       treasuryStatus,
    //       memberTokenAccount: memberTokenAccount.address,
    //       launchVault,
    //       tokenProgram: TOKEN_PROGRAM_ID,
    //       programAuthority,
    //       tokenMint: programTokenMint
    //     })
    //     .signers([payer])
    //     .prepare()

    //   console.log(tx.instruction)

    //   await program.methods
    //     .launch()
    //     .accountsPartial({
    //       member: payer.publicKey,
    //       memberStatus,
    //       treasuryStatus,
    //       memberTokenAccount: memberTokenAccount.address,
    //       launchVault,
    //       tokenProgram: TOKEN_PROGRAM_ID,
    //       programAuthority,
    //       tokenMint: programTokenMint
    //     })
    //     .signers([payer])
    //     .rpc();
    // })



  })



})

// it("Is initialized!", async () => {
//   // need usdc token mint

//   console.log(tokenMint.mint)

//   const tx = await program.methods
//     .initializeProgram()
//     .accounts({
//       usdcTokenMint: tokenMint.mint.publicKey,
//       // tokenProgram: TOKEN_2022_PROGRAM_ID,

//       // for testing, because can't figure out how to create ATA with TOKEN_2022_PROGRAM_ID,
//       tokenProgram: TOKEN_PROGRAM_ID,

//       usdcTokenProgram: TOKEN_PROGRAM_ID,
//     })
//     .rpc();
//   // .prepare();
//   console.log("Your transaction signature", tx);
// });

// it("Create Position Proposal!", async () => {

//   const [mockPoolStatePubkey] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("mock-pubkey")],
//     program.programId
//   )

//   const [proposalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("proposal-config")],
//     program.programId
//   )

//   const [positionProposal] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("0"),
//       Buffer.from("position-proposal"),
//     ],
//     program.programId
//   )

//   const tx = await program.methods
//     .createPositionProposal(new anchor.BN(1_000_000_000))
//     .accountsPartial({
//       // member
//       // memeber treasury status
//       // pool state
//       // position proposal
//       // proposal config
//       // sytem program
//       member: payer.publicKey,
//       poolState: mockPoolStatePubkey,
//       positionProposal,
//       proposalConfig,
//     })
//     .signers([payer])
//     .rpc();
//   console.log("Your transaction signature", tx);
// });

// // how this test is currently set up. it allows me to test the voting process
// // without worring about token issuance from launch vault
// it("transfer tokens!", async () => {
//   const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("authority")],
//     program.programId
//   )

//   const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       programAuthority.toBuffer(),
//       Buffer.from("dao-token-mint"),
//     ],
//     program.programId
//   )

//   const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       programAuthority.toBuffer(),
//       Buffer.from("launch-vault"),
//     ],
//     program.programId
//   )

//   const receipentATA = await getOrCreateAssociatedTokenAccount(
//     provider.connection, // connection
//     payer, // payer
//     programTokenMint, // mint
//     payer.publicKey,// owner
//   )

//   const tx = await program.methods
//     .transferTokens()
//     .accounts({
//       programAuthority,
//       vault: vault,
//       receipent: receipentATA.address,
//       tokenProgram: TOKEN_PROGRAM_ID,
//       // tokenProgram: TOKEN_2022_PROGRAM_ID,
//     })
//     .rpc();

//   console.log("Your transaction signature", tx);

// });

// it("Cast Vote", async () => {

//   const [programAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
//     [Buffer.from("authority")],
//     program.programId
//   )

//   const [programTokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       programAuthority.toBuffer(),
//       Buffer.from("dao-token-mint"),
//     ],
//     program.programId
//   )

//   const [positionProposal] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       Buffer.from("0"),
//       Buffer.from("position-proposal"),
//     ],
//     program.programId
//   )

//   const [memberVoteStatus] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       payer.publicKey.toBuffer(),
//       positionProposal.toBuffer(),
//       Buffer.from("member-vote-status"),
//     ],
//     program.programId
//   )

//   const [tokenMint] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       programAuthority.toBuffer(),
//       Buffer.from("dao-token-mint"),
//     ],
//     program.programId
//   )

//   const [ballotVault] = anchor.web3.PublicKey.findProgramAddressSync(
//     [
//       programAuthority.toBuffer(),
//       Buffer.from("ballot-vault"),
//     ],
//     program.programId
//   )

//   const memberTokenAccount = await getOrCreateAssociatedTokenAccount(
//     provider.connection, // connection
//     payer, // payer
//     programTokenMint, // mint
//     payer.publicKey,// owner
//   )

//   const tx = await program.methods
//     .castVote(new anchor.BN(1), 1, true)
//     .accountsPartial({
//       member: payer.publicKey,
//       memberVoteStatus,
//       programAuthority,
//       tokenMint,
//       ballotVault,
//       positionProposal,
//       memberTokenAccount: memberTokenAccount.address,
//       tokenProgram: TOKEN_PROGRAM_ID,
//       systemProgram: SYSTEM_PROGRAM_ID,
//     })
//     .signers([payer])
//     .rpc();

//   console.log("Your transaction signature", tx);
// })
