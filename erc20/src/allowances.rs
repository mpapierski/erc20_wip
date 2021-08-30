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

/// Creates a dictionary item key for a (owner, approver) pair.
fn make_dictionary_item_key(owner: &AccountHash, approver: &AccountHash) -> String {
    let key_string = format!("{}_{}", owner, approver);
    let key_bytes = runtime::blake2b(key_string);
    hex::encode(&key_bytes)
}

/// Writes an allowance for owner and approver for a specific amount.
pub fn write_allowance(owner: &AccountHash, approver: &AccountHash, amount: U512) {
    let allowance_uref = get_allowances_uref();
    let dictionary_item_key = make_dictionary_item_key(owner, approver);
    storage::dictionary_put(allowance_uref, &dictionary_item_key, amount)
}

/// Reads an allowance for a owner and approver
pub fn read_allowance(owner: &AccountHash, approver: &AccountHash) -> U512 {
    let allowance_uref = get_allowances_uref();
    let dictionary_item_key = make_dictionary_item_key(owner, approver);
    storage::dictionary_get(allowance_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_revert()
}
