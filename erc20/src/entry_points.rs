//! Contains definition of the entry points.
use alloc::vec;
use alloc::{string::String, vec::Vec};

use casper_types::U512;
use casper_types::{
    account::AccountHash, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Parameter,
};

use crate::constants::{
    ARG_ADDRESS, ARG_ADDRESSES, ARG_AMOUNT, ARG_OWNER, ARG_OWNER_AND_SPENDER_LIST, ARG_RECIPIENT,
    ARG_RECIPIENT_AND_AMOUNT_LIST, ARG_SPENDER, ARG_SPENDER_AND_AMOUNT_LIST, METHOD_ALLOWANCE,
    METHOD_APPROVE, METHOD_BALANCE_OF, METHOD_BATCH_ALLOWANCE, METHOD_BATCH_APPROVE,
    METHOD_BATCH_BALANCE_OF, METHOD_BATCH_TRANSFER, METHOD_DECIMALS, METHOD_NAME, METHOD_SYMBOL,
    METHOD_TRANSFER, METHOD_TRANSFER_FROM,
};

/// Returns entry points for an erc20 token.
pub fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    let name_entry_point = EntryPoint::new(
        String::from(METHOD_NAME),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let symbol_entry_point = EntryPoint::new(
        String::from(METHOD_SYMBOL),
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let decimals_entry_point = EntryPoint::new(
        String::from(METHOD_DECIMALS),
        vec![],
        u8::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let balance_of_entry_point = EntryPoint::new(
        String::from(METHOD_BALANCE_OF),
        vec![Parameter::new(ARG_ADDRESS, AccountHash::cl_type())],
        U512::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let batch_balance_of_entry_point = EntryPoint::new(
        String::from(METHOD_BATCH_BALANCE_OF),
        vec![Parameter::new(ARG_ADDRESSES, <Vec<AccountHash>>::cl_type())],
        <Vec<U512>>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let transfer_entry_point = EntryPoint::new(
        String::from(METHOD_TRANSFER),
        vec![
            Parameter::new(ARG_RECIPIENT, AccountHash::cl_type()),
            Parameter::new(ARG_AMOUNT, U512::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let batch_transfer_entry_point = EntryPoint::new(
        String::from(METHOD_BATCH_TRANSFER),
        vec![Parameter::new(
            ARG_RECIPIENT_AND_AMOUNT_LIST,
            <Vec<(AccountHash, U512)>>::cl_type(),
        )],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let approve_entry_point = EntryPoint::new(
        String::from(METHOD_APPROVE),
        vec![
            Parameter::new(ARG_SPENDER, AccountHash::cl_type()),
            Parameter::new(ARG_AMOUNT, U512::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let batch_approve_entry_point = EntryPoint::new(
        String::from(METHOD_BATCH_APPROVE),
        vec![Parameter::new(
            ARG_SPENDER_AND_AMOUNT_LIST,
            AccountHash::cl_type(),
        )],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let allowance_entry_point = EntryPoint::new(
        String::from(METHOD_ALLOWANCE),
        vec![
            Parameter::new(ARG_OWNER, AccountHash::cl_type()),
            Parameter::new(ARG_SPENDER, AccountHash::cl_type()),
        ],
        U512::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let batch_allowance_entry_point = EntryPoint::new(
        String::from(METHOD_BATCH_ALLOWANCE),
        vec![Parameter::new(
            ARG_OWNER_AND_SPENDER_LIST,
            <Vec<(AccountHash, AccountHash)>>::cl_type(),
        )],
        U512::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let transfer_from_entry_point = EntryPoint::new(
        String::from(METHOD_TRANSFER_FROM),
        vec![
            Parameter::new(ARG_OWNER, AccountHash::cl_type()),
            Parameter::new(ARG_RECIPIENT, AccountHash::cl_type()),
            Parameter::new(ARG_AMOUNT, U512::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let batch_transfer_from_entry_point = EntryPoint::new(
        String::from(METHOD_TRANSFER_FROM),
        vec![
            Parameter::new(ARG_OWNER, AccountHash::cl_type()),
            Parameter::new(ARG_RECIPIENT_AND_AMOUNT_LIST, AccountHash::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    entry_points.add_entry_point(name_entry_point);
    entry_points.add_entry_point(symbol_entry_point);
    entry_points.add_entry_point(decimals_entry_point);
    entry_points.add_entry_point(balance_of_entry_point);
    entry_points.add_entry_point(batch_balance_of_entry_point);

    entry_points.add_entry_point(transfer_entry_point);
    entry_points.add_entry_point(batch_transfer_entry_point);

    entry_points.add_entry_point(approve_entry_point);
    entry_points.add_entry_point(batch_approve_entry_point);

    entry_points.add_entry_point(allowance_entry_point);
    entry_points.add_entry_point(batch_allowance_entry_point);

    entry_points.add_entry_point(transfer_from_entry_point);
    entry_points.add_entry_point(batch_transfer_from_entry_point);

    entry_points
}
