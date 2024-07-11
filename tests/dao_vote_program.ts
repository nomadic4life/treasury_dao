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
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("dao_vote_program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DaoVoteProgram as Program<DaoVoteProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initializeProgram()
      .accounts({
        tokenProgram: TOKEN_2022_PROGRAM_ID,

      })
      .rpc();
    // .prepare();
    console.log("Your transaction signature", tx);
  });
});
