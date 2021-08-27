#![no_std]

pub mod allowances;
pub mod balances;
pub mod entry_points;

extern crate alloc;

use alloc::string::{String, ToString};
use core::convert::TryInto;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    {contracts::NamedKeys, CLTyped, CLValue, Key, URef, U256},
};

use crate::allowances::Allowances;
use crate::balances::Balances;

const NAME_KEY: &str = "name";
const SYMBOL_KEY: &str = "symbol";
const DECIMALS_KEY: &str = "decimals";
const CONTRACT_KEY: &str = "contract";
const BALANCES: &str = "balances";
const ALLOWANCES: &str = "allowances";
const TOTAL_SUPPLY: &str = "total_supply";

/// This is the main entry point of the contract.
///
/// It should be called from within `fn call` of your contract.
/// TODO: since it mentions `of your contract` we can perhaps turn `bin/main` into ./examples
pub fn delegate(name: String, symbol: String, decimals: u8, total_supply: U256) {
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

        let total_supply_key = {
            let total_supply_uref = storage::new_uref(total_supply).into_read();
            Key::from(total_supply_uref)
        };


        let balances_dictionary_key = {
            let balances_uref = storage::new_dictionary(BALANCES).unwrap_or_revert();
            storage::dictionary_put(
                balances_uref,
                &runtime::get_caller().to_string(),
               total_supply
            );
            Key::from(balances_uref)
        };

        let allowances_dictionary_key = {
            let allowance_uref = storage::new_dictionary(ALLOWANCES).unwrap_or_revert();
            Key::from(allowance_uref)
        };

        // Setup ERC20 named keys.
        named_keys.insert(NAME_KEY.to_string(), name_key);
        named_keys.insert(SYMBOL_KEY.to_string(), symbol_key);
        named_keys.insert(DECIMALS_KEY.to_string(), decimals_key);
        named_keys.insert(TOTAL_SUPPLY.to_string(), total_supply_key);

        // Store dictionary seed URefs in the ERC20 contract's named keys.
        named_keys.insert(BALANCES.to_string(), balances_dictionary_key);
        named_keys.insert(ALLOWANCES.to_string(), allowances_dictionary_key);

        named_keys
    };

    let (contract_hash, _version) =
        storage::new_locked_contract(entry_points, Some(named_keys), None, None);

    // Hash of the installed contract will be reachable through named keys.
    runtime::put_key(CONTRACT_KEY, Key::from(contract_hash));
}

pub mod interface {
    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use crate::utils::{get_balance_uref, get_allowances_uref};

    fn multiple_balance_of(balances: Balances, addresses: Vec<AccountHash>) -> Vec<U256> {
        let result = addresses
            .iter()
            .map(|account_hash| balances.read_balance(account_hash))
            .collect();
        result
    }

    pub fn balance_of(owner: AccountHash) -> U256 {
        let balance = get_balance_uref();
        let single_result = multiple_balance_of(Balances::new(balance), vec![owner]);
        single_result[0]
    }

    pub fn batch_balance_of(addresses: Vec<AccountHash>) -> Vec<U256> {
        let balance = get_balance_uref();
        multiple_balance_of(Balances::new(balance), addresses)
    }

    pub fn allowance(owner: AccountHash, spender: AccountHash) -> U256 {
        let allowance = get_allowances_uref();
        Allowances::new(allowance).read_allowances(&owner, &spender)
    }

    pub fn transfer(sender: AccountHash, recipient: AccountHash, amount: U256) {
        let balances_seed_uref: URef = get_balance_uref();
        let balance = Balances::new(balances_seed_uref);
        let new_sender_balance = balance.read_balance(&sender) - amount;
        balance.write_balance(&sender, new_sender_balance);
        let new_recipient_balance = balance.read_balance(&recipient) + amount;
        balance.write_balance(&recipient, new_recipient_balance)
    }

    pub fn approve(owner: AccountHash, approver: AccountHash, amount: U256) {
        let allowance_seed_uref: URef = get_allowances_uref();
        let allowance = Allowances::new(allowance_seed_uref);
        let reduced_allowance_amount = allowance.read_allowances(&owner, &approver) - amount;
        allowance.write_allowances(&owner, &approver, reduced_allowance_amount)
    }

    pub fn transfer_from(owner: AccountHash, recipient: AccountHash, amount: U256) {
        let approver = runtime::get_caller();
        transfer(owner, recipient, amount);
        approve(owner, approver, amount)
    }
}

mod utils {
    use super::*;

    pub fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
        match runtime::get_key(name) {
            None => Default::default(),
            Some(value) => {
                let key = value.try_into().unwrap_or_revert();
                storage::read(key).unwrap_or_revert().unwrap_or_revert()
            }
        }
    }

    pub fn get_balance_uref() -> URef {
        *runtime::get_key(BALANCES).unwrap().as_uref().unwrap()
    }

    pub fn get_allowances_uref() -> URef {
        *runtime::get_key(BALANCES).unwrap().as_uref().unwrap()
    }


    pub fn ret<T: CLTyped + ToBytes>(value: T) {
        runtime::ret(CLValue::from_t(value).unwrap_or_revert())
    }
}

pub use utils::{get_key, ret};

