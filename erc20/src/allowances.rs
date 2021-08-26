use alloc::{
    format,


    string::{String},
};


use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{account::AccountHash, URef, U256};



pub struct Allowances {
    seed_uref: URef,
}

impl Allowances {
    pub fn new(seed_uref: URef) -> Self {
        Allowances { seed_uref }
    }

    pub fn write_allowances(&self, owner: &AccountHash, approver: &AccountHash, amount: U256) {
        let dictionary_item_key = self.generate_dictionary_item_key(owner, approver);
        storage::dictionary_put(
            self.seed_uref,
            &dictionary_item_key,
            amount
        )
    }

    pub fn read_allowances(&self, owner: &AccountHash, approver: &AccountHash) -> U256 {
        let dictionary_item_key = self.generate_dictionary_item_key(owner, approver);
        storage::dictionary_get(
            self.seed_uref,
            &dictionary_item_key
        )
            .unwrap_or_revert()
            .unwrap_or_revert()
    }

    fn generate_dictionary_item_key(&self, owner: &AccountHash, approver: &AccountHash) -> String {
        let key_string = format!("{}_{}", owner, approver);
        hex::encode(runtime::blake2b(key_string))

    }
}
