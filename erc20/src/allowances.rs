//! Implementation of allowances.
use alloc::{format, string::String};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{account::AccountHash, URef, U512};

use crate::{constants::ALLOWANCES_KEY, detail};

fn get_allowances_uref() -> URef {
    detail::get_uref(ALLOWANCES_KEY)
}

/// Creates a dictionary item key for a (owner, spender) pair.
fn make_dictionary_item_key(owner: &AccountHash, spender: &AccountHash) -> String {
    let key_string = format!("{}_{}", owner, spender);
    let key_bytes = runtime::blake2b(key_string);
    hex::encode(&key_bytes)
}

/// Writes an allowance for owner and spender for a specific amount.
pub fn write_allowance(owner: &AccountHash, spender: &AccountHash, amount: U512) {
    let allowance_uref = get_allowances_uref();
    let dictionary_item_key = make_dictionary_item_key(owner, spender);
    storage::dictionary_put(allowance_uref, &dictionary_item_key, amount)
}

/// Reads an allowance for a owner and spender
pub fn read_allowance(owner: &AccountHash, spender: &AccountHash) -> U512 {
    let allowance_uref = get_allowances_uref();
    let dictionary_item_key = make_dictionary_item_key(owner, spender);
    storage::dictionary_get(allowance_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}
