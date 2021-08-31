#![no_std]
// #![cfg_attr(target_arch = "wasm32", no_main)]
#![no_main]

extern crate alloc;

use alloc::{string::String, vec::Vec};

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{account::AccountHash, U512};

use erc20::{
    constants::{
        ARG_ADDRESS, ARG_ADDRESSES, ARG_AMOUNT, ARG_DECIMALS, ARG_NAME, ARG_OWNER,
        ARG_OWNER_AND_SPENDER_LIST, ARG_RECIPIENT, ARG_RECIPIENT_AND_AMOUNT_LIST, ARG_SPENDER,
        ARG_SPENDER_AND_AMOUNT_LIST, ARG_SYMBOL, ARG_TOTAL_SUPPLY, DECIMALS_KEY, NAME_KEY,
        SYMBOL_KEY,
    },
    detail::{read_from, ret},
};

#[no_mangle]
pub extern "C" fn name() {
    let val: String = read_from(NAME_KEY);
    ret(val)
}

#[no_mangle]
pub extern "C" fn symbol() {
    let val: String = read_from(SYMBOL_KEY);
    ret(val)
}

#[no_mangle]
pub extern "C" fn decimals() {
    let val: u8 = read_from(DECIMALS_KEY);
    ret(val)
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: AccountHash = runtime::get_named_arg(ARG_ADDRESS);
    let val = erc20::balance_of(address);
    ret(val)
}

#[no_mangle]
pub extern "C" fn batch_balance_of() {
    let addresses: Vec<AccountHash> = runtime::get_named_arg(ARG_ADDRESSES);
    let val = erc20::batch_balance_of(addresses);
    ret(val)
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: AccountHash = runtime::get_named_arg(ARG_RECIPIENT);
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);

    erc20::transfer(recipient, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn batch_transfer() {
    let recipient_and_amount_list: Vec<(AccountHash, U512)> =
        runtime::get_named_arg(ARG_RECIPIENT_AND_AMOUNT_LIST);

    erc20::batch_transfer(recipient_and_amount_list).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: AccountHash = runtime::get_named_arg(ARG_SPENDER);
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);

    erc20::approve(spender, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn batch_approve() {
    let spender_and_amount_list: Vec<(AccountHash, U512)> =
        runtime::get_named_arg(ARG_SPENDER_AND_AMOUNT_LIST);

    erc20::batch_approve(spender_and_amount_list).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: AccountHash = runtime::get_named_arg(ARG_OWNER);
    let spender: AccountHash = runtime::get_named_arg(ARG_SPENDER);
    let val = erc20::allowance(owner, spender);
    ret(val)
}

#[no_mangle]
pub extern "C" fn batch_allowance() {
    let owner_and_spender_list: Vec<(AccountHash, AccountHash)> =
        runtime::get_named_arg(ARG_OWNER_AND_SPENDER_LIST);
    let val = erc20::batch_allowance(owner_and_spender_list);
    ret(val)
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: AccountHash = runtime::get_named_arg(ARG_OWNER);
    let recipient: AccountHash = runtime::get_named_arg(ARG_RECIPIENT);
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);
    erc20::transfer_from(owner, recipient, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn batch_transfer_from() {
    let owner: AccountHash = runtime::get_named_arg(ARG_OWNER);
    let recipient_and_amount_list: Vec<(AccountHash, U512)> =
        runtime::get_named_arg(ARG_RECIPIENT_AND_AMOUNT_LIST);
    erc20::batch_transfer_from(owner, recipient_and_amount_list).unwrap_or_revert();
}

#[no_mangle]
fn call() {
    let name: String = runtime::get_named_arg(ARG_NAME);
    let symbol: String = runtime::get_named_arg(ARG_SYMBOL);
    let decimals = runtime::get_named_arg(ARG_DECIMALS);
    let total_supply = runtime::get_named_arg(ARG_TOTAL_SUPPLY);

    erc20::delegate(name, symbol, decimals, total_supply);
}
