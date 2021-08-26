#![no_std]

pub mod entry_points;

extern crate alloc;

use alloc::string::{String, ToString};
use casper_contract::contract_api::{runtime, storage};
use casper_types::{contracts::NamedKeys, Key};

const NAME_KEY: &str = "name";
const SYMBOL_KEY: &str = "symbol";
const DECIMALS_KEY: &str = "decimals";
const CONTRACT_KEY: &str = "contract";

/// This is the main entry point of the contract.
///
/// It should be called from within `fn call` of your contract.
/// TODO: since it mentions `of your contract` we can perhaps turn `bin/main` into ./examples
pub fn delegate(name: String, symbol: String, decimals: u8) {
    let entry_points = entry_points::get_entry_points();

    let named_keys = {
        let mut named_keys = NamedKeys::new();

        let name_key = {
            let name_uref = storage::new_uref(name).into_read();
            Key::from(name_uref)
        };

        let symbol_key = {
            let symbol_uref = storage::new_uref(symbol).into_read();
            Key::from(symbol_uref)
        };

        let decimals_key = {
            let decimals_uref = storage::new_uref(decimals).into_read();
            Key::from(decimals_uref)
        };

        named_keys.insert(NAME_KEY.to_string(), name_key);
        named_keys.insert(SYMBOL_KEY.to_string(), symbol_key);
        named_keys.insert(DECIMALS_KEY.to_string(), decimals_key);

        named_keys
    };

    let (contract_hash, _version) =
        storage::new_locked_contract(entry_points, Some(named_keys), None, None);

    // Hash of the installed contract will be reachable through named keys.
    runtime::put_key(CONTRACT_KEY, Key::from(contract_hash));
}
