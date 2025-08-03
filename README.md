# Escrow Solana Smart Contract (Rust)
___
**Lightweight Solana based scrapper**, which helps to initiate escrow in crypto.
___
## Features
- Buyer can create an escrow specifying lamports (1000000000 lamps = 1 SOL), title and seller 
- Depending on a result buyer can either confirm or cancel an escrow
- If buyer does not respond, or seller does not have required products, it can cancel an escrow as well
- When escrow is closed lamports used to create an escrow account will be returned to buyer
- It's guaranteed  to double fund the same escrow
- Because escrow is closed after cancellation/confirmation it's 
- It's possible to have only 1 escrow with the same buyer and seller at a time
- It's possible to have unlimited amount of escrow for any buyer/seller at a time
- Any seller can be buyer, as well as any buyer can be a seller
- It's possible to issue escrow to yourself
___
## Modules
- `src/lib.rs` - main logic
- `tests/main.ts` - example tests
___
## Installation & Usage
1. [Install anchor and solana](https://www.anchor-lang.com/docs/installation)
2. Change `cluster` field under `[provider]` to any required (e.g. `mainnet`)
3. `anchor build && anchor deploy` - deploy to specified previously cluster
4. `anchor test --skip-local-validator --skip-build --skip-deploy` - to test smart contract
___
## Tech Stack
- [`anchor_lang`](https://github.com/solana-foundation/anchor)
- [`solana`](https://solana.com/)
