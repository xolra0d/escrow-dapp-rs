import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Main } from "../target/types/main";
import { Keypair } from "@solana/web3.js"
import { assert } from "chai";

describe("main", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider()

  const program = anchor.workspace.main as Program<Main>;
  const keypair = new Keypair();
  const amount = 10000000;
  const amount_bn = new anchor.BN(amount);

  it("Initialize", async () => {
    const tx = await program.methods
        .initializeEscrow(
            amount_bn
        ).accounts({
          signer: provider.publicKey,
        }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Proceed", async () => {
    const tx = await program.methods
        .confirmDelivery(
            amount_bn
        ).accounts({
          signer: provider.publicKey,
          recipient: keypair.publicKey,
        }).rpc();

    const account_balance = await provider.connection.getBalance(keypair.publicKey);
    assert.equal(amount, account_balance);

    console.log("Your transaction signature", tx);
  });
});
