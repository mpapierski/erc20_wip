//! Implementation details.
use core::convert::TryInto;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped, CLValue, URef,
};

/// Shortcut for `runtime::ret`
pub fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

/// Gets [`URef`] under a name.
pub fn get_uref(name: &str) -> URef {
    let key = runtime::get_key(name).unwrap_or_revert();
    key.try_into().unwrap_or_revert()
}

/// Reads value from a named key.
pub fn read_from<T>(name: &str) -> T
where
    T: FromBytes + CLTyped,
{
    let uref = get_uref(name);
    let value: T = storage::read(uref).unwrap_or_revert().unwrap_or_revert();
    value
}
