import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Main } from "../target/types/main";
import { Keypair } from "@solana/web3.js"

describe("main", () => {
    anchor.setProvider(anchor.AnchorProvider.env());
    const provider = anchor.getProvider()

    const program = anchor.workspace.main as Program<Main>;
    const receiver = new Keypair();

    const amount = 1_000_000_000;
    const amount_bn = new anchor.BN(amount);

    it("Initialize", async () => {
        const tx = await program.methods
            .createEscrow(0,
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
            .describeEscrow(0)
            .accounts({
                buyer: provider.publicKey,
                seller: receiver.publicKey,
            })
            .rpc()
        console.log("Your transaction signature", tx);
    });

    it("Confirm", async () => {
        const tx = await program.methods
            .confirmEscrow(0)
            .accounts({
                buyer: provider.publicKey,
                seller: receiver.publicKey,
            })
            .rpc();

        console.log("Your transaction signature", tx);
    });

    it("Initialize", async () => {
        const tx = await program.methods
            .createEscrow(0,
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
            .describeEscrow(0)
            .accounts({
                buyer: provider.publicKey,
                seller: receiver.publicKey,
            })
            .rpc()
        console.log("Your transaction signature", tx);
    });

    it("Cancel", async () => {
        const tx = await program.methods
            .cancelEscrow(0)
            .accounts({
                signer: provider.publicKey,
                buyer: provider.publicKey,
                seller: receiver.publicKey,
            })
            .rpc()
        console.log("Your transaction signature", tx);
    });

    it("Initialize error: Same ID", async () => {
        const tx1 = await program.methods
            .createEscrow(0,
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


        try {
            const _ = await program.methods
                .createEscrow(0,
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
        } catch (error) {
            if (error.message.includes("already in use")) {
                console.log("Failed as expected.")
            } else {
                console.log("Unexpected error: ", error.message)
            }
        }

        const cancel_escrow = await program.methods
            .cancelEscrow(0,
            ).accounts({
                buyer: provider.publicKey,
                seller: receiver.publicKey,
            })
            .rpc();
        console.log("Your cancel transaction signature", cancel_escrow);
    });
});
