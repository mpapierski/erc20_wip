use alloc::string::ToString;

use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{account::AccountHash, URef, U256};

pub struct Balances {
    seed_uref: URef,
}

impl Balances {
    pub fn new(seed_uref: URef) -> Self {
        Balances { seed_uref }
    }

    pub fn write_balance(&self, account_hash: &AccountHash, amount: U256) {
        storage::dictionary_put(self.seed_uref, &account_hash.to_string(), amount)
    }

    pub fn read_balance(&self, account_hash: &AccountHash) -> U256 {
        storage::dictionary_get(self.seed_uref, &account_hash.to_string())
            .unwrap_or_revert()
            .unwrap_or_default()
    }
}
