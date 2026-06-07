# NFT Staking (Metaplex Core + Anchor)

This repository contains a Solana Anchor program for in-place NFT staking utilizing Metaplex Core (MPL Core) assets. 

Unlike traditional staking which transfers assets to a program-owned vault, this project stakes NFTs in-place. It utilizes MPL Core's plugin system to freeze the asset inside the user's own wallet and calculates rewards dynamically upon unstaking.

## Core Features

- **In-place Staking**: Freezes the asset within the owner's wallet using the `FreezeDelegate` plugin.
- **Dynamic Reward Calculations**: Calculates token rewards based on daily rates (in basis points) and total elapsed days since staking.
- **Configurable Locking**: Enforces a minimum staking duration (freeze period) before an asset can be thawed and unstaked.

## Program Reference

- **Program ID**: `BDwdkG9VFFsT21iVGjqmnRuvEJzAQiYzsHTiiLrkx5as`
- **Key Accounts**:
  - `Config`: Staking configuration PDA derived via `["config", collection_address]`.
  - `Rewards Mint`: SPL Mint PDA used to pay out rewards, derived via `["rewards_mint", config_address]`.
  - `Update Authority`: Program-owned PDA that manages the collection's update permissions, derived via `["update_authority", collection_address]`.

## Setup & Testing

### Build
Compile the smart contracts:
```bash
anchor build
```

### Test
Integration tests require [Surfpool](https://surfpool.dev) to simulate time travel (advancing the cluster timestamp past the freeze period):

1. Start Surfpool in a separate terminal:
   ```bash
   surfpool start
   ```
2. Run the test suite:
   ```bash
   anchor test --skip-local-validator
   ```
