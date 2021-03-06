use hex;

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, AsymmetricType, CLTyped,
    ContractHash, Key, PublicKey, RuntimeArgs, U512,
};

const CONTRACT_KEY: &str = "contract";

pub mod erc20_args {
    pub const ARG_NAME: &str = "name";
    pub const ARG_SYMBOL: &str = "symbol";
    pub const ARG_DECIMALS: &str = "decimals";
    pub const ARG_TOTAL_SUPPLY: &str = "total_supply";
}

pub mod token_cfg {
    use super::*;
    pub const NAME: &str = "ERC20";
    pub const SYMBOL: &str = "ERC";
    pub const DECIMALS: u8 = 8;
    pub fn total_supply() -> U512 {
        1_000.into()
    }
}

fn blake2b256(item_key_string: &[u8]) -> Box<[u8]> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(item_key_string);
    hasher.finalize_boxed()
}

#[derive(Clone, Copy)]
pub struct Sender(pub AccountHash);

pub struct Token {
    context: TestContext,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
}

impl Token {
    pub fn deploy() -> Token {
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();

        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from("erc20.wasm");
        let session_args = runtime_args! {
            erc20_args::ARG_NAME => token_cfg::NAME,
            erc20_args::ARG_SYMBOL => token_cfg::SYMBOL,
            erc20_args::ARG_DECIMALS => token_cfg::DECIMALS,
            erc20_args::ARG_TOTAL_SUPPLY => token_cfg::total_supply()
        };

        let session = SessionBuilder::new(session_code, session_args)
            .with_address(ali.to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();

        context.run(session);
        Token {
            context,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash(),
        }
    }

    fn contract_hash(&self) -> ContractHash {
        self.context
            .get_account(self.ali)
            .unwrap()
            .named_keys()
            .get(CONTRACT_KEY)
            .unwrap()
            .normalize()
            .into_hash()
            .unwrap()
            .into()
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.ali, &[CONTRACT_KEY.to_string(), name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash().value(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    pub fn name(&self) -> String {
        self.query_contract("name").unwrap()
    }

    pub fn symbol(&self) -> String {
        self.query_contract("symbol").unwrap()
    }

    pub fn decimals(&self) -> u8 {
        self.query_contract("decimals").unwrap()
    }

    pub fn balance_of(&self, account: AccountHash) -> Option<U512> {
        let key = Key::Hash(self.contract_hash().value());
        let value = self
            .context
            .query_dictionary_item(key, Some("balances".to_string()), account.to_string())
            .ok()?;

        Some(value.into_t::<U512>().unwrap())
    }

    pub fn allowance(&self, owner: AccountHash, spender: AccountHash) -> Option<U512> {
        let mut preimage = [0; 64];
        preimage[..32].copy_from_slice(owner.as_bytes());
        preimage[32..].copy_from_slice(spender.as_bytes());
        let key_bytes = blake2b256(&preimage);
        hex::encode_to_slice(&key_bytes, &mut preimage).unwrap();

        let allowance_item_key = std::str::from_utf8(&preimage).unwrap().to_string();

        let key = Key::Hash(self.contract_hash().value());

        let value = self
            .context
            .query_dictionary_item(key, Some("allowances".to_string()), allowance_item_key)
            .ok()?;

        Some(value.into_t::<U512>().unwrap())
    }

    pub fn transfer(&mut self, recipient: AccountHash, amount: U512, sender: Sender) {
        self.call(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount
            },
        );
    }

    pub fn approve(&mut self, spender: AccountHash, amount: U512, sender: Sender) {
        self.call(
            sender,
            "approve",
            runtime_args! {
                "spender" => spender,
                "amount" => amount
            },
        );
    }

    pub fn transfer_from(
        &mut self,
        owner: AccountHash,
        recipient: AccountHash,
        amount: U512,
        sender: Sender,
    ) {
        self.call(
            sender,
            "transfer_from",
            runtime_args! {
                "owner" => owner,
                "recipient" => recipient,
                "amount" => amount
            },
        );
    }
}
