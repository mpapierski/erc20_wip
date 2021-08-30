#![no_std]
// #![cfg_attr(target_arch = "wasm32", no_main)]
#![no_main]

extern crate alloc;

use alloc::string::{String};
use casper_contract::contract_api::runtime;
use casper_types::{account::AccountHash, system::mint::TOTAL_SUPPLY_KEY, U512};

use erc20::{
    constants::{
        ARG_ADDRESS, ARG_AMOUNT, ARG_OWNER, ARG_RECIPIENT, ARG_SPENDER, DECIMALS_KEY, NAME_KEY,
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
pub extern "C" fn total_supply() {
    let val: U512 = read_from(TOTAL_SUPPLY_KEY);
    ret(val)
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let account: AccountHash = runtime::get_named_arg(ARG_ADDRESS);
    let val = erc20::balance_of(account);
    ret(val)
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: AccountHash = runtime::get_named_arg(ARG_OWNER);
    let spender: AccountHash = runtime::get_named_arg(ARG_SPENDER);
    let val = erc20::allowance(owner, spender);
    ret(val)
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: AccountHash = runtime::get_named_arg(ARG_SPENDER);
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);
    erc20::approve(runtime::get_caller(), spender, amount)
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: AccountHash = runtime::get_named_arg(ARG_RECIPIENT);
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);
    erc20::transfer(runtime::get_caller(), recipient, amount);
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: AccountHash = runtime::get_named_arg(ARG_OWNER);
    let recipient: AccountHash = runtime::get_named_arg(ARG_RECIPIENT);
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);
    erc20::transfer_from(owner, recipient, amount);
}

#[no_mangle]
fn call() {
    let name: String = runtime::get_named_arg(NAME_KEY);
    let symbol: String = runtime::get_named_arg(SYMBOL_KEY);
    let decimals = runtime::get_named_arg(DECIMALS_KEY);
    let total_supply = runtime::get_named_arg(TOTAL_SUPPLY_KEY);

    erc20::delegate(
        name,
        symbol,
        decimals,
        total_supply
    );
}
