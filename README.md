# Stellar Wrap - Smart Contract

> **The on-chain Soulbound Token (SBT) registry for Stellar Wrap. This contract stores non-transferable wrap records linked to user addresses, containing data hashes and persona archetypes.**

This repository contains the **Soroban smart contract** that serves as the on-chain anchor for Stellar Wrap. For the full application (frontend, backend, etc.), see the main Stellar Wrap repository.

---

## 📖 What is Stellar Wrap?

Stellar Wrap is a "Spotify Wrapped"-style experience built specifically for the Stellar community.

Block explorers are great for data, but terrible for stories. Stellar Wrap takes your raw, complex on-chain history—transactions, smart contract deployments, NFT buys—and transforms it into a beautiful, personalized visual story that anyone can understand and share.

By simply connecting your wallet, you get a dynamic snapshot of your month on Stellar, highlighting your achievements and assigning you a unique on-chain persona based on your activity.

**It's more than just stats; it's a tool for builders to prove their contributions and for users to flex their participation in the Stellar ecosystem.**

--- 

## 💡 Why We Need This

In Web3, your on-chain history is your resume, your identity, and your reputation. But right now, that reputation is hidden behind confusing transaction hashes.

**Stellar Wrap solves the visibility gap:**

- **For Builders & Developers:** It's hard to showcase the immense value of deploying open-source Soroban contracts. Stellar Wrap makes their code contributions visible and shareable to non-technical users.
- **For the Community:** We lack easy, viral loops to share excitement about what's happening on Stellar. This tool gives everyone a reason to post about their on-chain life on social media.
- **For Users:** It turns isolated transactions into a sense of progress and belonging within the ecosystem.

---

## 🚀 How the Contract Works

This smart contract provides the on-chain registry for Stellar Wrap records:

1.  **Initialize:** The contract is initialized once with an admin address that has permission to mint wrap records.
2.  **Mint Wrap:** The admin (backend service) calls `mint_wrap()` to create a soulbound record for a user, storing:

- Timestamp of when the wrap was generated
- SHA256 hash of the full off-chain JSON data (ensuring integrity)
- Archetype/persona assigned to the user (e.g., _"soroban_architect"_, _"defi_patron"_, _"diamond_hand"_)

3.  **Query:** Anyone can call `get_wrap()` to retrieve a user's wrap record, enabling verification and display of on-chain personas.
4.  **Soulbound:** Records are non-transferable (SBT), permanently linked to the user's Stellar address.

---

## 🎯 Key Metrics Tracked

We look beyond simple payments to capture the full spectrum of Stellar's vibrant ecosystem:

- **🧙‍♂️ Soroban Builder Stats:** Contracts deployed and unique user interactions. (Critical for developer reputation!).
- **🤝 dApp Interactions:** Which ecosystem projects did you support the most?
- **🎨 NFT Activity:** New mints collected and top creators supported.
- **💸 Network Volume:** A summary of your general transaction activity.
- **🏆 Your Monthly Persona:** A gamified badge that reflects your unique contribution style.

---

## Ecosystem Impact

This project is designed to support the growth of the Stellar network by:

1.  **Incentivizing Building:** Publicly celebrating developers who ship code creates positive reinforcement. A "Soroban Architect" badge is a social flex that encourages more building.
2.  **Driving Viral Activity:** Every shared Stellar Wrap card is organic marketing for the blockchain, showing the world that Stellar is active and being used.
3.  **Increasing Retention:** Giving users a personalized summary fosters a sense of ownership and encourages them to come back next month to beat their stats.

---

## 🏗️ Architecture

The diagram below shows how on-chain and off-chain components interact in the Stellar Wrap system:

```mermaid
sequenceDiagram
    participant Backend as Backend Service
    participant Admin as Admin Key
    participant User as User Wallet
    participant Contract as Soroban Contract
    participant Frontend as Frontend App

    Note over Backend: 1. Generate wrap data
    Backend->>Backend: Analyze user's on-chain activity
    Backend->>Backend: Compute data_hash (SHA256 of JSON)
    Backend->>Backend: Assign archetype persona

    Note over Backend,Admin: 2. Sign with admin key
    Backend->>Admin: Sign(contract_id + user + period + archetype + data_hash)
    Admin-->>Backend: Ed25519 signature

    Note over Backend,User: 3. Deliver to user
    Backend-->>User: signature + period + archetype + data_hash

    Note over User,Contract: 4. User claims on-chain
    User->>Contract: mint_wrap(user, period, archetype, data_hash, signature)
    Contract->>Contract: Verify user auth (require_auth)
    Contract->>Contract: Verify admin signature (ed25519_verify)
    Contract->>Contract: Check no duplicate (Wrap key)
    Contract->>Contract: Store WrapRecord (persistent)
    Contract->>Contract: Update balance & latest period
    Contract-->>User: Event: (mint, user, period) → archetype

    Note over Frontend,Contract: 5. Frontend reads data
    Frontend->>Contract: get_wrap(user, period)
    Contract-->>Frontend: WrapRecord {timestamp, data_hash, archetype, period}
    Frontend->>Contract: balance_of(user)
    Contract-->>Frontend: wrap count
    Frontend->>Frontend: Display persona & stats
```

---

## 📦 WASM Size Tracking

The CI pipeline enforces a WASM binary size limit to catch regressions before they reach production.

### How it works

Every push and pull request runs the `wasm-size` CI job, which:

1. Builds the contract in release mode (`--target wasm32-unknown-unknown`)
2. Reads the limit from `.github/wasm-size-limit` (in bytes)
3. Fails the build if the compiled WASM exceeds the limit
4. Reports current size, limit, and remaining headroom in the job summary
5. On pull requests, reports the size delta compared to the base branch
6. Uploads the WASM binary as a GitHub Actions artifact (90-day retention) for historical trending

### Updating the size limit

If you intentionally add features that increase the WASM size, update the limit file:

```bash
# Check current compiled size
cargo build --release --target wasm32-unknown-unknown
wc -c target/wasm32-unknown-unknown/release/stellar_wrap_contract.wasm

# Update the limit to new size + 20% buffer (example: 85000 bytes → 102000)
echo "102000" > .github/wasm-size-limit
```

Commit the updated `.github/wasm-size-limit` alongside your feature PR so reviewers can see the intentional size increase.

> **Hard limits:** Soroban enforces a maximum contract size of **256 KB** (262144 bytes). Keep the repo limit well below this to leave headroom for future features.

---

## 🔢 Named Constants

All magic numbers in the contract are defined in `src/constants.rs`:

| Constant | Value | Meaning |
|---|---|---|
| `LEDGERS_PER_DAY` | `17280` | Ledgers produced per day (~5 s block time) |
| `TTL_DAYS` | `365` | Default TTL period in days |
| `DEFAULT_TTL_LEDGERS` | `17280 × 365 = 6,307,200` | ~1-year TTL used for all persistent storage |
| `HASH_AND_KEY_LEN` | `32` | SHA-256 hash / Ed25519 public key length (bytes) |
| `SIGNATURE_LEN` | `64` | Ed25519 signature length (bytes) |
| `DEFAULT_COUNT` | `0` | Initial value for wrap count storage |
| `DEFAULT_LATEST_PERIOD` | `0` | Initial value for latest-period tracker |

---

## 🛠️ Tech Stack

- **Language:** Rust
- **Smart Contract Framework:** Soroban SDK v20.0.0
- **Build Tool:** Cargo
- **Target:** WebAssembly (WASM) for Soroban runtime
- **Testing:** Soroban SDK testutils

---

## 🗺️ Contract Features

- ✅ Admin-controlled initialization
- ✅ Soulbound token (SBT) minting with authorization checks
- ✅ Wrap record storage (timestamp, data hash, archetype)
- ✅ Public query interface for retrieving wrap records
- ✅ Event emission for minting actions
- ✅ Prevention of duplicate wraps per user
- ✅ Contract upgrade mechanism (admin-only WASM upgrade via `upgrade()`)

### Upgrading the Contract

The contract supports in-place WASM upgrades via Soroban's `update_current_contract_wasm`. All persistent storage (wrap records, admin key, etc.) is preserved across upgrades.

**Process:**
1. Upload the new WASM to the Stellar network and note its hash.
2. Call `upgrade(new_wasm_hash)` — requires admin authorization.
3. Soroban validates the hash against the uploaded blob and atomically replaces the code.

Only the admin address can trigger an upgrade. Any call without valid admin authorization will be rejected.

---

## 📊 Design Decision: On-Chain `WrapCount` and `balance_of`

**Issue [#40](https://github.com/zintarh/stellar-wrap-contract/issues/40) — Considered removing `WrapCount` storage**

### The trade-off

`WrapCount` is a persistent storage entry incremented on every `mint_wrap` call. This means every mint performs two persistent storage writes (the `WrapRecord` and the `WrapCount`). Since mints also emit events, the count *could* be derived off-chain by indexing those events.

### Decision: **Keep `WrapCount` and `balance_of`**

**Rationale:**

1. **On-chain composability.** `balance_of(user)` allows other Soroban contracts to read a user's wrap count in a single storage read. Removing it would make composability with future on-chain logic impossible without an expensive storage scan.
2. **Predictable cost.** One extra persistent write per mint is a fixed, bounded cost. Lazy counting via storage iteration would be unbounded and far more expensive at query time.
3. **Off-chain indexing is unreliable as a source of truth.** Events are not stored in contract state; an indexer can miss events or be unavailable. On-chain state is the canonical source of truth.

**Alternatives considered and rejected:**

| Option | Why rejected |
|---|---|
| Remove `WrapCount`, derive from events | Breaks on-chain composability; indexer dependency |
| Lazy count (iterate storage) | O(n) cost per query; prohibitively expensive at scale |
| Keep as-is | ✅ **Selected** — fixed cost, composable, canonical |

---

## 🔒 Mint Guard Storage Decision

The mint reentrancy guard uses Soroban temporary storage, not persistent storage.

- Temporary storage is cheaper and matches the guard lifecycle (single invocation scope).
- On successful mint, the guard key is removed explicitly.
- On failure paths (panic), the temporary entry is not persisted forever and is naturally cleaned up by Soroban TTL.

## 📝 Contract Interface

### Functions

- `initialize(e: Env, admin: Address)` - Initialize contract with admin (callable once)
- `mint_wrap(e: Env, to: Address, data_hash: BytesN<32>, archetype: Symbol)` - Mint a wrap record (admin only)
- `get_wrap(e: Env, user: Address) -> Option<WrapRecord>` - Retrieve a user's wrap record

### Storage

- `WrapRecord`: Contains `timestamp`, `data_hash`, and `archetype`
- `DataKey::Admin`: Stores the admin address
- `DataKey::Wrap(Address)`: Maps user addresses to their wrap records

### Error Codes

| Code | Error |
| --- | --- |
| 1 | `AlreadyInitialized` |
| 2 | `NotInitialized` |
| 3 | `Unauthorized` |
| 4 | `WrapAlreadyExists` |
| 5 | `InvalidSignature` |
