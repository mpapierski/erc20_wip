use casper_types::{account::AccountHash, EntryPoints, U512};

pub const ARG_ADDRESS: &str = "address";

pub fn balance_of(_address: AccountHash) -> U512 {
    U512::zero()
}

pub fn get_entry_points() -> EntryPoints {
    EntryPoints::new()
}
