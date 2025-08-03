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
        .createEscrow(
            {
              title: "Parser",
              lamports: amount_bn,
              seller: receiver.publicKey,
            }
        ).accounts({
          buyer: provider.publicKey,
        seller: receiver.publicKey,
        })
        .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Describe", async () => {
      const tx = await program.methods
          .describeEscrow()
          .accounts({
              buyer: provider.publicKey,
              seller: receiver.publicKey,
          })
          .rpc()
      console.log("Your transaction signature", tx);
  });

  it("Confirm", async () => {
    const tx = await program.methods
        .confirmEscrow()
        .accounts({
          buyer: provider.publicKey,
          seller: receiver.publicKey,
        })
        .rpc();

    console.log("Your transaction signature", tx);
  });

    it("Initialize", async () => {
        const tx = await program.methods
            .createEscrow(
                {
                    title: "Validator",
                    lamports: amount_bn,
                    seller: receiver.publicKey,
                }
            ).accounts({
                buyer: provider.publicKey,
                seller: receiver.publicKey,
            })
            .rpc();
        console.log("Your transaction signature", tx);
    });

    it("Describe", async () => {
        const tx = await program.methods
            .describeEscrow()
            .accounts({
                buyer: provider.publicKey,
                seller: receiver.publicKey,
            })
            .rpc()
        console.log("Your transaction signature", tx);
    });

    it("Cancel", async () => {
        const tx = await program.methods
            .cancelEscrow()
            .accounts({
                signer: provider.publicKey,
                buyer: provider.publicKey,
                seller: receiver.publicKey,
            })
            .rpc()
        console.log("Your transaction signature", tx);
    });
});
