//! Implementation of a ERC20 Token Standard.
#![warn(missing_docs)]
#![no_std]

#[macro_use]
extern crate alloc;

pub mod allowances;
pub mod balances;
pub mod constants;
pub mod detail;
pub mod entry_points;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use constants::{ALLOWANCES_KEY, BALANCES_KEY, CONTRACT_KEY, DECIMALS_KEY, NAME_KEY, SYMBOL_KEY};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{account::AccountHash, contracts::NamedKeys, Key, U512};

/// Returns name of the token.
pub fn name() -> String {
    detail::read_from(NAME_KEY)
}

/// Returns symbol of the token.
pub fn symbol() -> String {
    detail::read_from(SYMBOL_KEY)
}

/// Returns decimals of the token.
pub fn decimals() -> u8 {
    detail::read_from(DECIMALS_KEY)
}

/// Checks balance of an owner.
pub fn balance_of(owner: AccountHash) -> U512 {
    balances::read_balance(&owner)
}

/// Checks balance of multiple accounts at once.
pub fn batch_balance_of(addresses: Vec<AccountHash>) -> Vec<U512> {
    addresses
        .into_iter()
        .map(|account_hash| balances::read_balance(&account_hash))
        .collect()
}

/// Returns the amount allowed to spend.
pub fn allowance(owner: AccountHash, spender: AccountHash) -> U512 {
    allowances::read_allowance(&owner, &spender)
}

/// Transfer tokens from the `sender` to the `recipient`.
pub fn transfer(sender: AccountHash, recipient: AccountHash, amount: U512) {
    let sender_balance = balances::read_balance(&sender);

    let new_sender_balance = sender_balance - amount;
    balances::write_balance(&sender, new_sender_balance);

    let new_recipient_balance = balances::read_balance(&recipient) + amount;
    balances::write_balance(&recipient, new_recipient_balance)
}

/// Allow other address to transfer caller's tokens.
pub fn approve(owner: AccountHash, approver: AccountHash, amount: U512) {
    let allowance_amount = allowances::read_allowance(&owner, &approver);
    let reduced_allowance_amount = allowance_amount - amount;
    allowances::write_allowance(&owner, &approver, reduced_allowance_amount);
}

/// Transfer tokens from `owner` address to the `recipient` address if required `amount` was approved before to be spend by the direct caller.
///
/// This operation should decrement approved amount on the `owner`, and increase balance on the `recipient`.
pub fn transfer_from(owner: AccountHash, recipient: AccountHash, amount: U512) {
    let approver = runtime::get_caller();
    transfer(owner, recipient, amount);
    approve(owner, approver, amount)
}

/// This is the main entry point of the contract.
///
/// It should be called from within `fn call` of your contract.
/// TODO: since it mentions `of your contract` we can perhaps turn `bin/main` into ./examples
pub fn delegate(name: String, symbol: String, decimals: u8, initial_supply: U512) {
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

        let balances_dictionary_key = {
            let balances_uref = storage::new_dictionary(BALANCES_KEY).unwrap_or_revert();

            // Sets up initial balance for the caller.
            balances::write_balance(&runtime::get_caller(), initial_supply);

            runtime::remove_key(BALANCES_KEY);

            Key::from(balances_uref)
        };

        let allowances_dictionary_key = {
            let allowance_uref = storage::new_dictionary(ALLOWANCES_KEY).unwrap_or_revert();
            runtime::remove_key(ALLOWANCES_KEY);

            Key::from(allowance_uref)
        };

        named_keys.insert(NAME_KEY.to_string(), name_key);
        named_keys.insert(SYMBOL_KEY.to_string(), symbol_key);
        named_keys.insert(DECIMALS_KEY.to_string(), decimals_key);
        named_keys.insert(BALANCES_KEY.to_string(), balances_dictionary_key);
        named_keys.insert(ALLOWANCES_KEY.to_string(), allowances_dictionary_key);

        named_keys
    };

    let (contract_hash, _version) =
        storage::new_locked_contract(entry_points, Some(named_keys), None, None);

    // Hash of the installed contract will be reachable through named keys.
    runtime::put_key(CONTRACT_KEY, Key::from(contract_hash));
}
