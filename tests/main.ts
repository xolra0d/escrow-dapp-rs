import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Main } from "../target/types/main";
import { Keypair } from "@solana/web3.js"

describe("main", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider()

  const program = anchor.workspace.main as Program<Main>;
  const receiver = new Keypair();

  const amount = 1000000000;
  const amount_bn = new anchor.BN(amount);

  it("Initialize", async () => {
    const tx = await program.methods
        .initializeEscrow(
            {
              title: "Parser",
              lamports: amount_bn,
              seller: provider.publicKey,
              buyer: receiver.publicKey,
            }
        ).accounts({
          signer: provider.publicKey,
        })
        .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Proceed", async () => {
    const tx = await program.methods
        .confirmDelivery()
        .accounts({
          signer: provider.publicKey,
          recipient: receiver.publicKey,
        }).rpc();

    console.log("Your transaction signature", tx);
  });
});
