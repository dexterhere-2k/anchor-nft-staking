# Anchor Core Staking

An NFT staking program built with [Anchor](https://www.anchor-lang.com/) on Solana, using [Metaplex Core](https://developers.metaplex.com/core) (MPL Core) assets. NFTs are staked in-place using MPL Core's plugin system — the asset stays in the owner's wallet, frozen via a `FreezeDelegate` plugin, and earns token rewards proportional to staking time.

## How It Works

Instead of transferring NFTs to a vault, this program attaches plugins directly to the asset:

- **Stake** — Adds an `Attributes` plugin (recording `staked=true` and `staked_at=<timestamp>`) and a `FreezeDelegate` plugin (locking the asset in the owner's wallet).
- **Unstake** — Thaws the asset (sets `FreezeDelegate frozen=false`), resets the staking attributes, and mints reward tokens to the owner based on how long the NFT was staked.

Rewards are calculated as:

```
rewards = staked_days * rewards_bps * 10^decimals / 10000
```

## Program ID

```
BDwdkG9VFFsT21iVGjqmnRuvEJzAQiYzsHTiiLrkx5as
```

## Instructions

| Instruction | Description |
|---|---|
| `initialize` | Creates the staking config PDA and rewards mint for a collection |
| `create_collection` | Creates an MPL Core collection with the program's PDA as update authority |
| `mint_asset` | Mints an MPL Core NFT into the collection |
| `stake` | Stakes an NFT — freezes it and records the staking timestamp |
| `unstake` | Unstakes an NFT after the freeze period — thaws it and mints rewards |

## Accounts

### Config (PDA: `["config", collection]`)
| Field | Type | Description |
|---|---|---|
| `rewards_bps` | `u16` | Reward rate in basis points per day |
| `freeze_period` | `u16` | Minimum staking duration in days |
| `rewards_bump` | `u8` | Bump for the rewards mint PDA |
| `bump` | `u8` | Bump for this config PDA |

## PDAs

| Seed | Description |
|---|---|
| `["config", collection]` | Staking config per collection |
| `["rewards_mint", config]` | SPL token mint for staking rewards |
| `["update_authority", collection]` | Program-owned update authority for the collection |

## Prerequisites

- [Rust](https://rustup.rs/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) `0.31.1`
- [Surfpool](https://surfpool.dev) (required for time-travel tests)
- Node.js + Yarn

## Build

```bash
anchor build
```

## Test

Tests use [Surfpool](https://surfpool.dev) for `surfnet_timeTravel` (advancing the validator clock to simulate the freeze period elapsing).

**Terminal 1** — Start Surfpool:
```bash
surfpool start
```

**Terminal 2** — Run the test suite:
```bash
anchor test --skip-local-validator
```

### Test Results

![All tests passing](.anchor-core-staking/tests.png)

## Project Structure

```
programs/anchor-core-staking/src/
├── instructions/
│   ├── create_collection.rs  # Create MPL Core collection
│   ├── initialize.rs         # Initialize staking config + rewards mint
│   ├── mint_asset.rs         # Mint NFT into collection
│   ├── stake.rs              # Stake NFT (freeze + record attributes)
│   └── unstake.rs            # Unstake NFT (thaw + mint rewards)
├── state/
│   └── config.rs             # Config account struct
├── error.rs                  # Custom error codes
└── lib.rs                    # Program entrypoint
tests/
└── anchor-core-staking.ts    # Integration tests
```

## Dependencies

- `anchor-lang` `0.31.1`
- `anchor-spl` `0.31.1`
- `mpl-core` `0.11.1`
