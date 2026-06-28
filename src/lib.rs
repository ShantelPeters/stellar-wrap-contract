#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, panic_with_error, Address, Env, IntoVal, Symbol, Val,
    Vec,
};

pub mod constants;
mod storage_types;
use constants::{
    Sha256Hash, DEFAULT_COUNTER_VALUE, DEFAULT_TTL_LEDGERS, LEDGERS_PER_DAY, USER_COUNT_INCREMENT,
};
use storage_types::{DataKey, WrapRecord};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    WrapAlreadyExists = 4,
    InvalidSignature = 5,
    InvalidArchetype = 6,
}

#[contract]
pub struct StellarWrapContract;

#[contractimpl]
impl StellarWrapContract {
    pub fn initialize(e: Env, admin: Address) {
        extend_instance_ttl(&e);

        let key = DataKey::Admin;

        if e.storage().instance().has(&key) {
            panic_with_error!(e, ContractError::AlreadyInitialized);
        }

        e.storage().instance().set(&key, &admin);
        e.storage()
            .instance()
            .set(&DataKey::AllowedArchetypes, &default_archetypes(&e));
    }

    pub fn mint_wrap(
        e: Env,
        to: Address,
        data_hash: Sha256Hash,
        archetype: Symbol,
        period: Symbol,
    ) {
        extend_instance_ttl(&e);

        let admin = get_admin_or_panic(&e);
        admin.require_auth();

        if !verify_signature(&data_hash) {
            panic_with_error!(e, ContractError::InvalidSignature);
        }

        if !is_allowed_archetype(&e, &archetype) {
            panic_with_error!(e, ContractError::InvalidArchetype);
        }

        let wrap_key = DataKey::Wrap(to.clone(), period.clone());
        if e.storage().instance().has(&wrap_key) {
            panic_with_error!(e, ContractError::WrapAlreadyExists);
        }

        let minted_at = e.ledger().timestamp();

        let record = WrapRecord {
            minted_at,
            data_hash,
            archetype: archetype.clone(),
            period,
        };

        e.storage().instance().set(&wrap_key, &record);

        let user_count_key = DataKey::UserCount(to.clone());
        let user_count: u32 = e
            .storage()
            .instance()
            .get(&user_count_key)
            .unwrap_or(DEFAULT_COUNTER_VALUE);
        e.storage()
            .instance()
            .set(&user_count_key, &(user_count + USER_COUNT_INCREMENT));

        use soroban_sdk::symbol_short;

        let topics: Vec<Val> =
            Vec::from_array(&e, [symbol_short!("mint").into_val(&e), to.into_val(&e)]);

        let period_u64 = symbol_to_u64(&record.period);
        e.events().publish(topics, period_u64);
    }

    pub fn get_wrap(e: Env, user: Address, period: Symbol) -> Option<WrapRecord> {
        extend_instance_ttl(&e);

        let wrap_key = DataKey::Wrap(user, period);
        e.storage().instance().get(&wrap_key)
    }

    pub fn get_user_count(e: Env, user: Address) -> u32 {
        extend_instance_ttl(&e);

        let user_count_key = DataKey::UserCount(user);
        e.storage()
            .instance()
            .get(&user_count_key)
            .unwrap_or(DEFAULT_COUNTER_VALUE)
    }

    pub fn get_admin(e: Env) -> Address {
        extend_instance_ttl(&e);
        get_admin_or_panic(&e)
    }

    pub fn add_archetype(e: Env, archetype: Symbol) {
        extend_instance_ttl(&e);

        let admin = get_admin_or_panic(&e);
        admin.require_auth();

        let mut archetypes = get_allowed_archetypes_or_default(&e);
        if !archetypes.contains(&archetype) {
            archetypes.push_back(archetype);
            e.storage()
                .instance()
                .set(&DataKey::AllowedArchetypes, &archetypes);
        }
    }

    pub fn remove_archetype(e: Env, archetype: Symbol) {
        extend_instance_ttl(&e);

        let admin = get_admin_or_panic(&e);
        admin.require_auth();

        let mut archetypes = get_allowed_archetypes_or_default(&e);
        if let Some(index) = archetypes.first_index_of(&archetype) {
            archetypes.remove(index);
            e.storage()
                .instance()
                .set(&DataKey::AllowedArchetypes, &archetypes);
        }
    }

    pub fn get_allowed_archetypes(e: Env) -> Vec<Symbol> {
        extend_instance_ttl(&e);
        get_allowed_archetypes_or_default(&e)
    }

    pub fn upgrade(e: Env, wasm_hash: Sha256Hash) {
        let admin = get_admin_or_panic(&e);
        admin.require_auth();
        e.deployer().update_current_contract_wasm(wasm_hash);
    }
}

fn get_admin_or_panic(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::Admin)
        .unwrap_or_else(|| panic_with_error!(e, ContractError::NotInitialized))
}

fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGERS_PER_DAY, DEFAULT_TTL_LEDGERS);
}

fn default_archetypes(e: &Env) -> Vec<Symbol> {
    use soroban_sdk::symbol_short;

    Vec::from_array(
        e,
        [
            symbol_short!("builder"),
            symbol_short!("architect"),
            symbol_short!("defi"),
            symbol_short!("patron"),
        ],
    )
}

fn get_allowed_archetypes_or_default(e: &Env) -> Vec<Symbol> {
    e.storage()
        .instance()
        .get(&DataKey::AllowedArchetypes)
        .unwrap_or_else(|| default_archetypes(e))
}

fn is_allowed_archetype(e: &Env, archetype: &Symbol) -> bool {
    get_allowed_archetypes_or_default(e).contains(archetype)
}

fn verify_signature(_data_hash: &Sha256Hash) -> bool {
    true
}

fn symbol_to_u64(symbol: &Symbol) -> u64 {
    let val: Val = symbol.to_val();
    val.get_payload()
}

#[cfg(test)]
mod test;
