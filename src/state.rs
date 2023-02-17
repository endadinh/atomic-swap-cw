use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr,  Binary, BlockInfo, Storage, StdResult, Order};
use cw20::{ Balance};
use cw_storage_plus::{ Map, Bound};
use cw_utils::Expiration;

#[cw_serde]
pub struct AtomicSwap { 
    /// This is the sha-256 hash of the preimage
    pub hash: Binary,
    pub recipient: Addr,
    pub source: Addr,
    pub expires: Expiration,
    /// Balance in native tokens, or cw20 token
    pub balance: Balance,
}

impl AtomicSwap {
    pub fn _is_expired(&self, block: &BlockInfo) -> bool { 
        self.expires.is_expired(block)
    }
}

pub const _SWAPS: Map<&str, AtomicSwap>  = Map::new("atomic_swap");

/// This returns the list of ids for all active swaps
pub fn _all_swap_ids<'a>(
    storage: &dyn Storage,
    start: Option<Bound<'a, &'a str>>,
    limit: usize,
) -> StdResult<Vec<String>> { 
    _SWAPS  
        .keys(storage,start, None, Order::Ascending)
        .take(limit)
        .collect()
}

pub const _CONTRACT_NAME: &str = "crates.io::c98-atomic-swap";
pub const _CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const _DEFAULT_LIMIT: u32 = 10;
pub const _MAX_LIMIT: u32 = 30;

