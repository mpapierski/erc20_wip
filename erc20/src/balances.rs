//! Implementation of balances.
use alloc::string::String;

use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{account::AccountHash, URef, U512};

use crate::{constants::BALANCES_KEY, detail};

/// Creates a dictionary item key for a dictionary item.
fn make_dictionary_item_key(owner: &AccountHash) -> String {
    format!("{}", owner)
}

fn get_balances_uref() -> URef {
    detail::get_uref(BALANCES_KEY)
}

/// Writes token balance of a specified account.
pub fn write_balance(account_hash: &AccountHash, amount: U512) {
    let balances_uref = get_balances_uref();
    let dictionary_item_key = make_dictionary_item_key(account_hash);

    storage::dictionary_put(balances_uref, &dictionary_item_key, amount);
}

/// Reads token balance of a specified account.
///
/// If a given account does not have balances in the system, then a 0 is returned.
pub fn read_balance(account_hash: &AccountHash) -> U512 {
    let balances_uref = get_balances_uref();
    let dictionary_item_key = make_dictionary_item_key(account_hash);

    storage::dictionary_get(balances_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}
