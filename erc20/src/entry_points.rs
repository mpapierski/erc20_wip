use alloc::vec;
use alloc::{string::String, vec::Vec};

use casper_types::{
    account::AccountHash, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Parameter,
};

pub const ARG_ADDRESS: &str = "address";

pub fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(create_entrypoint("name", vec![], CLType::String));
    entry_points.add_entry_point(create_entrypoint("symbol", vec![], CLType::String));
    entry_points.add_entry_point(create_entrypoint("decimals", vec![], CLType::String));
    entry_points.add_entry_point(create_entrypoint("total_supply", vec![], CLType::String));
    entry_points.add_entry_point(create_entrypoint(
        "transfer",
        vec![
            Parameter::new("recipient", AccountHash::cl_type()),
            Parameter::new("amount", CLType::U512),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(create_entrypoint(
        "approve",
        vec![
            Parameter::new("sender", AccountHash::cl_type()),
            Parameter::new("amount", CLType::U512),
        ],
        CLType::Unit,
    ));
    entry_points.add_entry_point(create_entrypoint(
        "allowance",
        vec![
            Parameter::new("owner", AccountHash::cl_type()),
            Parameter::new("spender", AccountHash::cl_type()),
        ],
        CLType::U512,
    ));
    entry_points.add_entry_point(create_entrypoint(
        "transfer_from",
        vec![
            Parameter::new("owner", AccountHash::cl_type()),
            Parameter::new("recipient", AccountHash::cl_type()),
            Parameter::new("amount", CLType::U512),
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
