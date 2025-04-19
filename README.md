# 🐙 Octopus: CIDMapper

<p align="center">
  <img src="./docs/assets/octopus-logo.png" alt="Octopus: CIDMapper Logo" width="200" />
</p>

<p align="center">
  <strong>Dynamic NFT metadata evolution for Soroban smart contracts</strong>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#getting-started">Getting Started</a> •
  <a href="#documentation">Documentation</a> •
  <a href="#examples">Examples</a> •
  <a href="#contributing">Contributing</a> •
  <a href="#license">License</a>
</p>

## 📋 Overview

Octopus is an open-source framework for the Soroban/Stellar ecosystem that enables dynamic NFT metadata management using the InterPlanetary CID Mapping (IPCM) pattern. It elegantly separates on-chain ownership (NFTs) from off-chain content (IPFS) while maintaining verifiable links between them through smart contracts.

Like its namesake animal, Octopus connects multiple systems intelligently while adapting to changing environments. It allows metadata to evolve over time without modifying the underlying NFT, using blockchain events for efficient synchronization.

## ✨ Features

- **Decoupled Architecture**: Separate NFT contract from IPCM content mapping
- **Event-Driven Updates**: Emit and listen for blockchain events to track metadata changes
- **Complete History**: Maintain immutable record of all metadata versions
- **Flexible Integration**: Connect with any IPFS provider
- **Optimized Storage**: Store only what's needed on-chain, keep content off-chain
- **Powerful Indexing**: Query the latest or historical NFT metadata quickly
- **Stellar/Soroban Native**: Built specifically for the Soroban smart contract platform

## 🏗️ Architecture

Octopus consists of four main components:

1. **NFT Contract**: Handles token minting, ownership, and transfers
   - Stores minimal on-chain data (token ID, reference to IPCM contract)
   - Standard NFT functionality (mint, transfer, burn)

2. **IPCM Contract**: Manages mappings between token IDs and IPFS CIDs
   - Updates mappings when metadata changes
   - Emits events for all updates
   - Maintains references to current CIDs

3. **Indexer Service**: Monitors blockchain events
   - Subscribes to IPCM contract events
   - Builds searchable database of metadata history
   - Provides API for applications

4. **Client SDK**: Simplifies integration
   - Manages IPFS uploads
   - Interfaces with both contracts
   - Handles event subscription

## 🚀 Getting Started

### Prerequisites

- Soroban CLI
- Rust toolchain
- Node.js (for indexer service)
- IPFS node or provider account (e.g., Pinata)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/octopus.git
cd octopus

# Install dependencies
cargo build
cd indexer && npm install
```

### Quick Start

```bash
# Deploy the contracts
cargo run --bin deploy

# Start the indexer
cd indexer && npm start

# Run the example app
cd examples/basic && npm start
```

## 📖 Documentation

### Core Concepts

#### NFT and Metadata Separation

Octopus separates the concerns of token ownership from token metadata:

1. The NFT itself contains only ownership information and a reference to its metadata in the IPCM contract
2. The IPCM contract maps token IDs to current IPFS CIDs
3. The actual metadata is stored on IPFS

This separation allows for:
- Efficient on-chain storage
- Metadata evolution without modifying the NFT
- Complete historical record of changes

#### Event-Driven Architecture

When metadata is updated:
1. New content is uploaded to IPFS, generating a new CID
2. The IPCM contract updates the mapping and emits an event
3. The indexer service captures this event and updates its database
4. Applications can query the latest or historical metadata through the indexer

## 💡 Examples

### Basic NFT with Evolving Metadata

```rust
// Deploy the IPCM contract
let ipcm_address = deploy_ipcm_contract(&env, &admin);

// Deploy the NFT contract with reference to IPCM
let nft_address = deploy_nft_contract(&env, &admin, &ipcm_address);

// Create initial metadata and upload to IPFS
let initial_cid = ipfs_upload(initial_metadata);

// Initialize IPCM mapping
let token_id = "TOKEN123";
update_ipcm_mapping(&env, &ipcm_address, &admin, token_id, initial_cid);

// Mint the NFT
mint_nft(&env, &nft_address, &admin, token_id, &owner);

// Later, update metadata
let updated_cid = ipfs_upload(updated_metadata);
update_ipcm_mapping(&env, &ipcm_address, &admin, token_id, updated_cid);
```

### Query Current Metadata

```javascript
// Using the JavaScript SDK
const Octopus = require('octopus-cidmapper');

const octopus = new Octopus({
  nftContract: 'CONTRACT_ADDRESS',
  ipcmContract: 'CONTRACT_ADDRESS',
  indexerUrl: 'https://indexer.example.com'
});

// Get the latest metadata
const metadata = await octopus.getMetadata('TOKEN123');
console.log(metadata);

// Get historical versions
const history = await octopus.getMetadataHistory('TOKEN123');
history.forEach(version => {
  console.log(`Version from ${version.timestamp}: ${version.cid}`);
});
```

## 🔍 Use Cases

- **Supply Chain**: Track products through manufacturing and distribution
- **Credentials**: Educational certificates and professional licenses
- **Real Estate**: Property records with ownership and modification history
- **Collectibles**: Digital items with evolving characteristics
- **Governance**: Organizational records with amendment history
- **Art**: Creative works with evolving attributes or appearances

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Soroban Team](https://soroban.stellar.org/) for the smart contract platform
- [IPFS](https://ipfs.io/) for the distributed file system
- [Pinata](https://pinata.cloud/) for IPCM inspiration and IPFS tooling

---

<p align="center">
  Built with ❤️ by the Octopus Team
</p>