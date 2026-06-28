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

## 📝 Contract Interface

### Functions

- `initialize(e: Env, admin: Address)` - Initialize contract with admin (callable once)
- `mint_wrap(e: Env, to: Address, data_hash: BytesN<32>, archetype: Symbol, period: Symbol)` - Mint a wrap record for a period (admin only)
- `get_wrap(e: Env, user: Address, period: Symbol) -> Option<WrapRecord>` - Retrieve a user's wrap record for a period
- `get_user_count(e: Env, user: Address) -> u32` - Retrieve the number of wraps minted for a user
- `get_admin(e: Env) -> Address` - Retrieve the configured admin address
- `add_archetype(e: Env, archetype: Symbol)` - Add an allowed archetype (admin only)
- `remove_archetype(e: Env, archetype: Symbol)` - Remove an allowed archetype (admin only)
- `get_allowed_archetypes(e: Env) -> Vec<Symbol>` - Retrieve the current archetype allowlist
- `upgrade(e: Env, wasm_hash: BytesN<32>)` - Upgrade the current contract WASM (admin only)

### Storage

- `WrapRecord`: Contains `minted_at`, `data_hash`, `archetype`, and `period`
- `DataKey::Admin`: Stores the admin address
- `DataKey::Wrap(Address, Symbol)`: Maps user addresses and periods to their wrap records
- `DataKey::UserCount(Address)`: Tracks the number of wraps minted for a user
- `DataKey::AllowedArchetypes`: Stores the admin-managed archetype allowlist

## Archetype Validation

Archetypes remain stored as `Symbol` values for backwards compatibility with existing wraps. Replacing the field with a contract enum would reduce storage variability, but it would be a breaking storage migration because records already serialized with `Symbol` would no longer decode cleanly.

The contract therefore uses an admin-managed allowlist. `initialize()` seeds the list with the known short archetypes `builder`, `architect`, `defi`, and `patron`; admins can update it with `add_archetype()` and `remove_archetype()`. `mint_wrap()` rejects archetypes that are not present in the allowlist.

## Testnet Deployment

The `.github/workflows/deploy-testnet.yml` workflow deploys automatically on pushes to `main` and can also be run manually with `workflow_dispatch`.

Required GitHub Actions secret:

- `STELLAR_DEPLOYER_SECRET`: secret key for the funded Stellar testnet deployer account.

Optional GitHub Actions secret:

- `STELLAR_TESTNET_CONTRACT_ID`: existing testnet contract ID. When present, the workflow installs the new WASM and calls `upgrade()` instead of deploying a fresh contract.

Manual dispatch inputs:

- `contract_id`: overrides `STELLAR_TESTNET_CONTRACT_ID` for an ad-hoc upgrade.
- `admin_address`: admin address used when initializing a new deployment. Defaults to the deployer public key.
- `initialize`: whether to call `initialize()` after a fresh deployment.

Every deployment writes `contract-id.txt` as a GitHub Actions artifact and adds the contract ID plus `get_admin()` smoke-test result to the job summary.
