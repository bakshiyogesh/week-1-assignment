# Solana Rust 

A Rust client project for practicing SPL tokens and Metaplex Core assets on Solana Devnet.

It demonstrates:

- Creating an SPL token mint, creating associated token accounts (ATAs), minting tokens, and transferring tokens.
- Adding Metaplex Token Metadata to an SPL mint.
- Creating an NFT with MPL Core, updating its name and metadata URI, transferring ownership, and burning the asset.

This is an educational client. It calls existing Solana programs; it does not deploy a custom on-chain program.

## Requirements

- Rust and Cargo (Rust 1.98.1 or newer for the locked dependencies).
- Solana CLI and SPL Token CLI.
- A Solana wallet configured for Devnet with devnet SOL for transaction fees and account rent.
- Publicly reachable JSON metadata and image URLs. This project currently uses GitHub raw URLs; update the URIs in the source if you use your own files.

Check the CLI setup:

```bash
solana --version
solana config get
solana address
solana balance
```

Set the cluster to Devnet and fund the configured wallet from a Devnet faucet if needed:

```bash
solana config set --url devnet
solana airdrop 2
```

The RPC URL is defined in `src/constant.rs`.

## Clone and build

```bash
git clone <REPOSITORY_URL>
cd solana-assignment-rust
cargo build
```

The repository intentionally does not include wallet or asset keypairs. Create local keypairs as needed from the project directory:

```bash
solana-keygen new --no-bip39-passphrase --outfile mint-keypair.json
solana-keygen new --no-bip39-passphrase --outfile new-nft-keypair.json
```

The `.gitignore` excludes these files. Never commit or share keypair files, private keys, or recovery phrases. A fresh keypair creates a new mint or Core asset address; it cannot control an asset created by a different keypair.

The wallet used to pay and sign is loaded from `~/.config/solana/id.json` by the sample code. Update that path if your CLI uses a different keypair.

## Choose the action to run

`src/main.rs` is the action selector. Enable only the function you intend to run, then use `cargo run`:

**In this checkout, `main.rs` currently calls `core_nft::burn_core_nft()`. Change that call before running `cargo run` unless you intend to burn the asset configured in `core_nft.rs`.**

```rust
fn main() {
    // token::create_and_transfer_spl_token();
    // core_nft::create_core_nft();
    // core_nft::update_core_nft();
    // core_nft::transfer_core_nft();
    // core_nft::burn_core_nft();
}
```

For example, to create a Core NFT, enable only `core_nft::create_core_nft();`. To update it later, enable only `core_nft::update_core_nft();`.

> **Important:** `burn_core_nft()` permanently burns the asset. Only enable it when you intend to destroy the selected NFT. The asset owner must sign. The burn transaction's fee payer receives most of the rent refund; a small minimum remains in the account.

Before any transaction, compile first:

```bash
cargo check
```

## SPL token workflow

The SPL functions are in `src/token.rs`. The mint keypair is loaded from `mint-keypair.json`; the configured wallet is the mint authority and fee payer. The token uses six decimals.

The token workflow contains transaction operations, so do not rerun it casually. Minting increases supply each time; transferring moves existing tokens. The ATA create instruction is idempotent, but submitting it still costs a transaction fee. Inspect `create_and_transfer_spl_token()` and enable the intended steps in order for a fresh Devnet mint:

1. Create and initialize the mint.
2. Create the owner's ATA.
3. Mint an initial supply to that ATA.
4. Transfer tokens to the recipient's ATA.
5. Create token metadata once, if desired.

The SPL token metadata uses Metaplex Token Metadata. It is separate from the MPL Core NFT metadata.

## MPL Core NFT workflow

The Core functions are in `src/core_nft.rs`. The asset keypair is loaded from `new-nft-keypair.json`; its public key is the asset address. The wallet from `id.json` pays and is set as owner/update authority when creating the sample asset.

- `create_core_nft()` creates a new asset. Use a fresh asset keypair for a new asset; rerunning with an existing asset keypair is guarded and skips creation.
- `update_core_nft()` changes the asset name and metadata URI. The current update authority must sign.
- `transfer_core_nft()` prompts for the new owner's Solana address. The current owner must sign; the recipient does not need to sign to receive.
- `burn_core_nft()` permanently destroys the asset. The current owner or a burn delegate must authorize it.

The asset's metadata JSON and image are hosted off-chain. The Core account stores the URI; make sure it points to a public JSON file, and that the JSON's image URL is publicly accessible.

## Verify Devnet results

Save transaction signatures and public addresses for assignment evidence. You can inspect addresses with the Solana CLI:

```bash
solana account <MINT_OR_ASSET_ADDRESS> --url devnet
```

For SPL balances:

```bash
spl-token balance <MINT_ADDRESS> --owner <WALLET_ADDRESS> --url devnet
```

After a Core transfer, the recipient becomes the owner and must authorize later transfers or burns. After a burn, the asset cannot be restored.
