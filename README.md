<img width="1919" height="1002" alt="image" src="https://github.com/user-attachments/assets/b68be305-93d2-4137-86f2-d59cda40c09f" /># 🌳 TreeCert — On-Chain NFT Certificates for Verified Tree Planting

> A Soroban smart contract on the Stellar blockchain that mints a unique NFT certificate for every verified tree planted — turning environmental action into a permanent, tradeable on-chain record.

---

## 📖 Project Description

TreeCert bridges real-world environmental impact with blockchain transparency. Every time a trusted verifier confirms that a tree has been planted and is alive, the contract mints a non-fungible certificate (NFT) assigned to the planter or any designated recipient. The certificate is stored permanently on Stellar's ledger and can be transferred, displayed, or used as proof-of-impact in green finance, carbon credit, or CSR workflows.

---

## ⚙️ What It Does

1. **Verifier onboarding** — An admin registers trusted entities (NGOs, field agents, IoT sensors) as authorised verifiers.
2. **Certificate minting** — A verifier calls `mint_certificate` with the planter's details, GPS location, species, planting timestamp, and an IPFS URI pointing to photo/data evidence. The contract creates a unique `TreeCertificate` struct and assigns the next sequential `token_id`.
3. **Ownership tracking** — Each address's holdings are indexed on-chain, so `tokens_of(owner)` returns all their certificates instantly.
4. **Transfer** — Certificates can be transferred between wallets (useful for corporate offset purchases, gifting, or secondary markets).
5. **Querying** — Anyone can read a certificate's full details, check total supply, or list an address's portfolio.

---

## ✨ Features

| Feature | Details |
|---|---|
| 🔐 **Role-based access** | Only admin-approved verifiers can mint; admin manages the verifier set |
| 🪙 **Sequential token IDs** | Auto-incrementing `token_id` starting at 1 |
| 🗺️ **Rich metadata on-chain** | Species, GPS coordinates, planting & verification timestamps stored directly in contract storage |
| 🖼️ **Off-chain media via IPFS** | `metadata_uri` links to photos/extended data without bloating the ledger |
| 🔄 **Transferable certificates** | Full ownership transfer with automatic index updates |
| 📡 **Events** | `mint` and `transfer` events emitted for easy indexing by explorers / dApps |
| 🔍 **Query functions** | `get_certificate`, `tokens_of`, `total_supply`, `admin`, `verifiers` |
| 🧪 **Test-ready** | `testutils` feature flag for unit testing with the Soroban test SDK |

---

## 🏗️ Project Structure

```
tree-nft-contract/
├── Cargo.toml        # Rust workspace & Soroban SDK dependency
└── src/
    └── lib.rs        # Full contract — data types, storage, logic
```

---

## 🚀 Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the WASM target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

### Build

```bash
stellar contract build
# Output: target/wasm32-unknown-unknown/release/tree_nft_contract.wasm
```

### Deploy (Testnet)

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/tree_nft_contract.wasm \
  --source <YOUR_SECRET_KEY> \
  --network testnet
```

### Initialize

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <ADMIN_SECRET> \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS>
```

### Add a Verifier

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <ADMIN_SECRET> \
  --network testnet \
  -- add_verifier \
  --verifier <VERIFIER_ADDRESS>
```

### Mint a Certificate

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <VERIFIER_SECRET> \
  --network testnet \
  -- mint_certificate \
  --verifier <VERIFIER_ADDRESS> \
  --planter <PLANTER_ADDRESS> \
  --recipient <RECIPIENT_ADDRESS> \
  --species '"Ficus benghalensis"' \
  --location '"lat:28.6139,lon:77.2090"' \
  --planted_at 1714000000 \
  --metadata_uri '"ipfs://bafybeig..."'
```

### Query a Certificate

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- get_certificate \
  --token_id 1
```

---

## 🔐 Access Control Summary

| Role | Capabilities |
|---|---|
| **Admin** | Add/remove verifiers, mint certificates |
| **Verifier** | Mint certificates |
| **Token Owner** | Transfer their own certificates |
| **Anyone** | Read all certificate data and supply |

---

## 🗺️ Roadmap

- [ ] Batch minting for large reforestation events
- [ ] On-chain carbon credit score per certificate
- [ ] Soroban token interface (SEP-0041) compliance
- [ ] Expiry / health-check re-verification mechanic
- [ ] Frontend dApp with Freighter wallet integration

---

wallet address = GD5JNH5TTCWSBTY47Q5IHHJ4PI57364LUPQIFKCGZ4TBSYFF6UAF7FKI

contract address = CB4AZ7ODIQKVIMIRRKLYFUC4GGAYFJHVSCVDMNMO6D7G6P7SIYKAVG5M 

https://stellar.expert/explorer/testnet/contract/CB4AZ7ODIQKVIMIRRKLYFUC4GGAYFJHVSCVDMNMO6D7G6P7SIYKAVG5M

<img width="1919" height="1002" alt="image" src="https://github.com/user-attachments/assets/524a6ec2-8fe5-4783-ac60-70ce2e267b76" />

