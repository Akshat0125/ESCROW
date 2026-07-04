# 🔐 Escrow — Trustless SPL Token Swap on Solana

A minimal, trustless escrow program built with the [Anchor](https://www.anchor-lang.com/) framework on Solana. It lets two parties (a **maker** and a **taker**) swap SPL tokens without needing to trust each other or a third-party intermediary — the program itself holds the funds and only releases them when both sides of the trade are satisfied.

> Inspired by the classic Anchor escrow pattern: Alice deposits Token A and specifies how much Token B she wants; Bob comes along, sends the Token B, and receives the Token A automatically — atomically, in a single transaction.

---

## ✨ Features

- **Trustless swaps** — funds are held in a Program Derived Address (PDA)-owned vault, not by either party.
- **Three simple instructions** — `make`, `take`, and `refund` cover the entire lifecycle of an escrow.
- **Atomic settlement** — the token exchange and vault cleanup happen in a single `take` transaction; nothing can be partially executed.
- **Maker-initiated cancellation** — the maker can reclaim their deposit at any time via `refund`, as long as the escrow hasn't been taken.
- **Rent reclamation** — the vault and escrow state accounts are closed automatically after a swap or refund, returning rent to the maker.

## ⚙️ How It Works

1. **Make** — Alice (the *maker*) creates an escrow, depositing `deposit_amount` of **Token A** into a program-owned vault (an ATA controlled by the escrow's PDA), and records how much **Token B** she wants in return (`receive_amount`).
2. **Take** — Bob (the *taker*) accepts the trade:
   - Bob sends `receive_amount` of Token B directly to Alice.
   - The program releases the full Token A balance from the vault to Bob.
   - The vault is closed and its rent is refunded to Alice.
3. **Refund** — If Alice changes her mind before anyone takes the trade, she can call `refund` to withdraw her Token A from the vault. The vault and escrow state are closed, and rent is returned to her.

```
        make                         take
Alice ────────► [Escrow PDA + Vault] ────────► Bob
 (deposits A)      (holds Token A)      (sends B → Alice, receives A)

        refund (maker-only, before take)
Alice ◄──────── [Escrow PDA + Vault]
 (reclaims A)
```

## 🏗️ Program Architecture

### State

| Account | Description |
|---|---|
| `EscrowState` | PDA storing `maker`, `mint_a`, `mint_b`, `receive_amount`, and the PDA `bump`. Seeded by `["escrow", maker, seed]`. |

### Instructions

| Instruction | Signer | Description |
|---|---|---|
| `make(seed, receive_amount, deposit_amount)` | Maker | Initializes the escrow state + vault, transfers `deposit_amount` of Token A from the maker into the vault. |
| `take()` | Taker | Transfers `receive_amount` of Token B to the maker, releases all vaulted Token A to the taker, closes the vault and escrow state. |
| `refund()` | Maker | Returns all vaulted Token A to the maker and closes the vault and escrow state. |

### Program ID

```
3EXJ5DHYdqNungWGWR2S9wNMMucCr2pRVjPitfLFDfTi
```

## 📁 Project Structure

```
ESCROW/
├── Anchor.toml                       # Anchor workspace config (localnet, yarn)
├── Cargo.toml                        # Rust workspace manifest
├── rust-toolchain.toml               # Pinned Rust toolchain (1.89.0)
├── package.json                      # JS/TS tooling (prettier, mocha, chai)
├── migrations/
│   └── deploy.ts                     # Anchor deploy script
└── programs/
    └── escrow_prc/
        ├── Cargo.toml
        ├── src/
        │   ├── lib.rs                # Program entrypoint (make / take / refund)
        │   ├── instructions/
        │   │   ├── make.rs           # Create escrow + deposit Token A
        │   │   ├── take.rs           # Execute the swap
        │   │   ├── refund.rs         # Cancel escrow, return deposit
        │   │   └── mod.rs
        │   └── state/
        │       ├── escrow.rs         # EscrowState account definition
        │       └── mod.rs
        └── tests/
            └── test_initialize.rs    # Rust test scaffold
```

## 🧰 Tech Stack

- **[Anchor](https://www.anchor-lang.com/)** (`anchor-lang`, `anchor-spl`) — Solana program framework
- **Rust** `1.89.0` (pinned via `rust-toolchain.toml`)
- **SPL Token / Associated Token Account** program for token transfers
- **[LiteSVM](https://github.com/LiteSVM/litesvm)** — lightweight SVM for fast Rust-native program tests
- **TypeScript / Yarn** — deployment tooling

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) `1.89.0` (managed automatically via `rust-toolchain.toml`)
- [Solana CLI](https://docs.solanalabs.com/cli/install)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)
- [Yarn](https://yarnpkg.com/)

### Clone & Install

```bash
git clone https://github.com/Akshat0125/ESCROW.git
cd ESCROW
yarn install
```

### Build

```bash
anchor build
```

### Test

Tests run directly through Cargo (see `[scripts]` in `Anchor.toml`):

```bash
anchor test
# or
cargo test
```

### Deploy (localnet)

```bash
solana-test-validator   # in a separate terminal
anchor deploy
```

## 🔒 Security Notes

- The vault is an ATA whose **authority is the `EscrowState` PDA** — no private key can move funds out except through the program's own instructions.
- `take` and `refund` both use `has_one` constraints to ensure the correct maker/mint accounts are supplied, and PDA `seeds`/`bump` checks to prevent account substitution.
- This program is a learning/demo implementation and **has not been audited**. Do not use it to hold real value on mainnet without a professional security review.

## 🗺️ Roadmap

- [ ] Flesh out integration tests (`test_initialize.rs` is currently a placeholder)
- [ ] Add partial-fill / multi-taker support
- [ ] Add dispute/arbitration flow
- [ ] TypeScript client SDK + example scripts

## 📄 License

Licensed under the **ISC License** (see `package.json`).

## 👤 Author

**Akshat** ([@Akshat0125](https://github.com/Akshat0125))
