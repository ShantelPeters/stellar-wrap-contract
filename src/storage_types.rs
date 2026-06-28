use core::fmt;

use soroban_sdk::{contracttype, Address, Symbol};

use crate::constants::{Sha256Hash, HASH_PREVIEW_BYTES};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WrapRecord {
    pub minted_at: u64,
    pub data_hash: Sha256Hash,
    pub archetype: Symbol,
    pub period: Symbol,
}

impl fmt::Display for WrapRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hash = self.data_hash.to_array();

        write!(
            f,
            "WrapRecord {{ period: {:?}, archetype: {:?}, minted_at: {}, data_hash: ",
            self.period, self.archetype, self.minted_at
        )?;

        for byte in hash.iter().take(HASH_PREVIEW_BYTES) {
            write!(f, "{byte:02x}")?;
        }

        write!(f, "... }}")
    }
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Wrap(Address, Symbol),
    UserCount(Address),
    AllowedArchetypes,
}
