//! Contains definition of the entry points.
use alloc::vec;
use alloc::{string::String, vec::Vec};

use casper_types::{
    account::AccountHash, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Parameter,
};

use crate::constants::{
    ARG_AMOUNT, ARG_OWNER, ARG_RECIPIENT, ARG_SPENDER, METHOD_ALLOWANCE, METHOD_APPROVE,
    METHOD_DECIMALS, METHOD_NAME, METHOD_SYMBOL, METHOD_TOTAL_SUPPLY, METHOD_TRANSFER,
    METHOD_TRANSFER_FROM,
};

/// Returns entry points for an erc20 token.
pub fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(create_entrypoint(METHOD_NAME, vec![], String::cl_type()));
    entry_points.add_entry_point(create_entrypoint(METHOD_SYMBOL, vec![], String::cl_type()));
    entry_points.add_entry_point(create_entrypoint(METHOD_DECIMALS, vec![], u8::cl_type()));
    entry_points.add_entry_point(create_entrypoint(
        METHOD_TOTAL_SUPPLY,
        vec![],
        CLType::String,
    ));
    entry_points.add_entry_point(create_entrypoint(
        METHOD_TRANSFER,
        vec![
            Parameter::new(ARG_RECIPIENT, AccountHash::cl_type()),
            Parameter::new(ARG_AMOUNT, CLType::U512),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(create_entrypoint(
        METHOD_APPROVE,
        vec![
            Parameter::new(ARG_SPENDER, AccountHash::cl_type()),
            Parameter::new(ARG_AMOUNT, CLType::U512),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(create_entrypoint(
        METHOD_ALLOWANCE,
        vec![
            Parameter::new(ARG_OWNER, AccountHash::cl_type()),
            Parameter::new(ARG_SPENDER, AccountHash::cl_type()),
        ],
        CLType::U512,
    ));
    entry_points.add_entry_point(create_entrypoint(
        METHOD_TRANSFER_FROM,
        vec![
            Parameter::new(ARG_OWNER, AccountHash::cl_type()),
            Parameter::new(ARG_RECIPIENT, AccountHash::cl_type()),
            Parameter::new(ARG_AMOUNT, CLType::U512),
        ],
        CLType::Unit,
    ));
    entry_points
}

fn create_entrypoint(name: &str, params: Vec<Parameter>, ret: CLType) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        params,
        ret,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
