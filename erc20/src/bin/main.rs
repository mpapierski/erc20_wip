#![no_std]
// #![cfg_attr(target_arch = "wasm32", no_main)]
#![no_main]

extern crate alloc;

use alloc::string::ToString;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{account::AccountHash, CLValue};
use erc20::entry_points::{self, ARG_ADDRESS};

#[no_mangle]
extern "C" fn balance_of() {
    let address: AccountHash = runtime::get_named_arg(ARG_ADDRESS);

    let ret = entry_points::balance_of(address);

    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn call() {
    // TODO: pass name, symbol and decimals as named args?
    erc20::delegate("Token".to_string(), "TOK".to_string(), 255);
}
