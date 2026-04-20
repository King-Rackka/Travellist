# ✈️ Travellist
**Travellist — Blockchain-Based Decentralized Travel Bucket List**

---

## Project Description

Travellist DApp is a decentralized smart contract solution built on the **Stellar blockchain** using the **Soroban SDK**. It provides a secure, immutable platform for managing your personal travel bucket list directly on the blockchain. The contract ensures that your destination data is stored transparently and is only manageable through predefined smart contract functions, eliminating reliance on centralized database providers.

The system allows users to add, view, toggle, and delete travel destinations, leveraging the efficiency and security of the Stellar network. Each destination is uniquely identified, categorized, and stored within the contract's instance storage — ensuring data persistence and reliability across sessions.

---

## Project Vision

Our vision is to revolutionize how travelers manage their dream destinations in the digital age by:

- **Decentralizing Data:** Moving travel lists from centralized apps to a global, distributed blockchain
- **Ensuring Ownership:** Empowering users to have complete control and ownership over their travel goals
- **Guaranteeing Immutability:** Providing a permanent, tamper-proof record of destinations that cannot be altered by third parties
- **Enhancing Privacy:** Leveraging blockchain security to protect personal travel information from unauthorized access
- **Building Trustless Systems:** Creating a platform where data integrity is guaranteed by code, not by company promises

We envision a future where your personal travel bucket list is truly yours — sovereign, permanent, and always accessible.

---

## Key Features

### 1. Add Destination
- Add travel destinations with a single function call
- Specify name, country, category, and priority level
- Automated ID generation for unique identification
- Persistent storage on the Stellar blockchain

### 2. Efficient Data Retrieval
- Fetch all stored destinations in a single call
- Structured data representation for easy frontend integration
- Quick access to your entire bucket list
- Real-time synchronization with blockchain state

### 3. Toggle Visited Status
- Mark any destination as visited or unvisited
- Tracks your real-world travel progress on-chain
- Instant update of visited status without deletion
- Reflects your journey as it grows over time

### 4. Secure Deletion
- Remove specific destinations using their unique IDs
- Permanent removal from contract storage
- Clean and efficient storage management
- Immediate update of the destination list after deletion

### 5. On-Chain Travel Statistics
- Count total destinations stored on-chain
- Count how many destinations have been visited
- Enables progress tracking directly from the contract
- Useful for frontend dashboards and analytics

### 6. Category & Priority System
- Organize destinations by category: Pantai, Gunung, Kota, Budaya, Alam, Kuliner
- Set priority levels: High, Mid, Low
- Structured enum types enforced at the contract level
- Makes filtering and sorting straightforward on the frontend

### 7. Stellar Network Integration
- Leverages the high speed and low cost of Stellar
- Built using the modern Soroban Smart Contract SDK
- Scalable architecture for growing destination collections
- Interoperable with other Stellar-based services

---

## Contract Details

**Contract Address (Testnet):**
```
CCHTMTFDP6QP653JLDBBNMXLTBU4RRVEEMOQOW3QSPTEW5YZUCATUY4I
```
> Replace with your actual Contract ID after deploying to Stellar Testnet.

| Property | Value |
|---|---|
| Network | Stellar Testnet |
| RPC URL | `https://soroban-testnet.stellar.org` |
| Network Passphrase | `Test SDF Network ; September 2015` |
| Explorer | [stellar.expert/explorer/testnet](https://stellar.expert/explorer/testnet) |

> **Screenshot (Testnet Explorer):**  
> _(Paste your Stellar Expert or Freighter wallet screenshot here after deployment)_

---

## Data Structure

```rust
pub struct Destination {
    id: u64,             // Auto-generated unique ID
    name: String,        // Destination name, e.g. "Raja Ampat"
    country: String,     // Country or region, e.g. "West Papua, Indonesia"
    category: Category,  // Enum: Pantai | Gunung | Kota | Budaya | Alam | Kuliner
    priority: Priority,  // Enum: High | Mid | Low
    visited: bool,       // false = dream, true = visited
}
```

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) + `wasm32-unknown-unknown` target
- [Soroban CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/soroban-cli)
- Stellar Testnet account funded with XLM via [Friendbot](https://friendbot.stellar.org)

### Build & Deploy

```bash
# 1. Clone the repository
git clone https://github.com/yourusername/travellist
cd travellist

# 2. Build the contract
soroban contract build

# 3. Deploy to Testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/travellist.wasm \
  --source YOUR_SECRET_KEY \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

---

## Contract Functions

Deploy the smart contract to Stellar's Soroban network and interact with it using these main functions:

- `add_destination()` — Add a new destination with name, country, category, and priority
- `get_destinations()` — Retrieve all stored destinations from the contract
- `toggle_visited()` — Mark a destination as visited or unvisited by ID
- `delete_destination()` — Remove a specific destination by its ID
- `count_total()` — Get the total number of saved destinations
- `count_visited()` — Get the number of destinations already visited

---

## Future Scope

### Short-Term Enhancements
- **Search & Filter:** Filter destinations by category or priority on-chain
- **Notes per Destination:** Add personal notes or travel tips to each destination
- **Rich Metadata:** Support for estimated budget, planned date, and travel duration

### Medium-Term Development
- **Multi-User Support:** Per-wallet bucket lists using address-based storage keys
- **Collaborative Lists:** Shared lists with multi-signature access for travel partners
- **Frontend Integration:** React + Freighter wallet interface for seamless UX

### Long-Term Vision
- **Cross-Chain Sync:** Extend destination storage to multiple blockchain networks
- **Decentralized UI Hosting:** Host the frontend on IPFS or similar decentralized platforms
- **AI Travel Suggestions:** Optional AI integration to suggest destinations based on visited history
- **DAO Governance:** Community-driven feature prioritization for the protocol

---

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar blockchain network

---

## Project Structure

```
travellist/
├── src/
│   ├── lib.rs       # Smart contract (TravellistContract)
│   └── test.rs      # Unit tests
├── Cargo.toml
└── README.md
```

---

## License

MIT License © 2025 — **Travellist DApp** — Securing Your Dreams on the Blockchain.