# Squiggle - Onchain Generative NFT

Squiggle is a fully onchain generative NFT collection built on Arbitrum using Stylus and Rust. Each NFT generates a unique squiggly line pattern as SVG art that is encoded directly into the token's metadata - no external dependencies or IPFS required.

## Features

- **Fully Onchain**: All art generation and metadata happens onchain
- **Generative Art**: Each NFT creates unique squiggly patterns using deterministic randomness
- **ERC721 Compatible**: Standard NFT functionality with minting, transfers, and metadata
- **Dynamic Pricing**: Owner can adjust mint price
- **SVG Generation**: Creates scalable vector graphics with smooth bezier curves
- **Colorful Gradients**: Three different gradient themes (rainbow, sunset, ocean)

## How It Works

When an NFT is minted:
1. A unique seed is generated using block data, sender address, and chain ID
2. The seed determines visual parameters:
   - Number of oscillations (4-15)
   - Stroke width (10-80px)
   - X offsets and Y coordinates for each curve
   - Gradient color scheme
3. An SVG is generated with smooth bezier curves connecting the points
4. The SVG is base64 encoded into JSON metadata
5. The complete metadata is returned as a data URI

## Contract Functions

### Public Functions
- `mint()` - Mint a new Squiggle NFT (payable)
- `tokenURI(uint256)` - Get the complete metadata for a token
- `name()` - Returns "Squiggle"
- `symbol()` - Returns "SQGL"

### Owner Functions
- `update_mint_price(uint256)` - Update the mint price
- `get_contract_balance()` - Check contract ETH balance

### Standard ERC721
All standard ERC721 functions are supported via OpenZeppelin integration.

## Quick Start

### Prerequisites

Install [Rust](https://www.rust-lang.org/tools/install) and the Stylus CLI:

```bash
cargo install --force cargo-stylus cargo-stylus-check
rustup target add wasm32-unknown-unknown
```

### Build and Test

```bash
# Run tests
cargo test

# Check Stylus compatibility
cargo stylus check

# Export ABI
cargo stylus export-abi
```

### Deploy

```bash
# Estimate gas
cargo stylus deploy --private-key-path=<PRIVKEY_FILE_PATH> --estimate-gas

# Deploy to testnet
cargo stylus deploy --private-key-path=<PRIVKEY_FILE_PATH>
```

## Project Structure

- `src/lib.rs` - Main contract implementation with ERC721 functionality
- `src/generator.rs` - SVG generation logic and art parameters
- `src/base64.rs` - Custom base64 encoding for onchain use

## Art Generation Details

Each Squiggle is generated with:
- **Oscillations**: 4-15 curve segments creating the squiggly pattern
- **Stroke Width**: 10-80px thickness
- **Curves**: Smooth cubic bezier curves with calculated control points
- **Gradients**: Three themes - rainbow, sunset, or ocean colors
- **Dimensions**: 1000x1000px SVG on dark background

The generation uses the first 32 bytes of the seed to determine all visual parameters, ensuring each NFT is unique and deterministic.

## Testnet Information

Deploy and test on Arbitrum Stylus testnet. Find faucets and RPC endpoints at the [official Arbitrum docs](https://docs.arbitrum.io/stylus/reference/testnet-information).

## License

This project is open source under Apache-2.0 or MIT license.
