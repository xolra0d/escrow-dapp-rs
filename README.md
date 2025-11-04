# Escrow Solana Smart Contract (Rust)
___
**Lightweight Solana based smart-contract**, which helps to initiate escrow in crypto.
___
## Features
- Buyer can create an escrow specifying lamports (1_000_000_000 lamports = 1 SOL) with minimum 1 lamport, title (at max 128 characters) and seller. 
- Depending on a result buyer can either confirm or cancel an escrow.
- If buyer does not respond, or seller does not have required products, it can cancel an escrow.
- When escrow is closed lamports used to create an escrow account will be returned to the buyer
- It's possible to have u32::MAX escrows with the same buyer and seller at a time
- It's possible to have unlimited amount of escrow for any buyer/seller at a time
- Any seller can be buyer, as well as any buyer can be a seller
- It's possible to issue escrow to yourself
___
## Methods
- `createEscrow(escrow_id, DealInfo)` - initiate escrow
- `describe_escrow(escrow_id)` - give escrow information
- `confirm_escrow(escrow_id)` - resolve escrow by transferring lamports to seller
- `cancel_escrow(escrow_id)` - resolve escrow by returning lamports to buyer
## Modules
- `src/lib.rs` - main logic
- `tests/main.ts` - example tests
___
## Installation & Usage
1. [Install anchor and solana](https://www.anchor-lang.com/docs/installation)
2. Change `cluster` field under `[provider]` to any required (e.g. `mainnet`)
3. `anchor build && anchor deploy` - deploy to specified previously cluster
4. `anchor test` - test smart contract
___
## Tech Stack
- [`anchor_lang`](https://github.com/solana-foundation/anchor)
- [`solana`](https://solana.com/)
