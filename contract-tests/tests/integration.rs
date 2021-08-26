use casper_engine_test_support::{
    Code, SessionBuilder, TestContext, TestContextBuilder, DEFAULT_ACCOUNT_ADDR,
};
use casper_types::RuntimeArgs;

const CONTRACT_NAME: &str = "erc20.wasm";

const NAME_KEY: &str = "name";
const SYMBOL_KEY: &str = "symbol";
const DECIMALS_KEY: &str = "decimals";
const CONTRACT_KEY: &str = "contract";

fn setup() -> TestContext {
    let mut test_context = TestContextBuilder::default().build();

    let install_request = SessionBuilder::new(Code::from(CONTRACT_NAME), RuntimeArgs::default())
        .with_address(*DEFAULT_ACCOUNT_ADDR)
        .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
        .build();

    test_context.run(install_request);

    let account_value = test_context
        .query(*DEFAULT_ACCOUNT_ADDR, &[])
        .expect("should query default account");
    let account = account_value.into_account().expect("should have account");

    assert!(
        account.named_keys().contains_key(CONTRACT_KEY),
        "installing contract should add named key"
    );

    test_context
}

#[test]
fn should_have_description_properties() {
    let test_context = setup();

    let name: String = test_context
        .query(
            *DEFAULT_ACCOUNT_ADDR,
            &[CONTRACT_KEY.to_string(), NAME_KEY.to_string()],
        )
        .expect("should query name property of the contract")
        .into_t()
        .expect("should be string");
    assert_eq!(name, "Token");

    let symbol: String = test_context
        .query(
            *DEFAULT_ACCOUNT_ADDR,
            &[CONTRACT_KEY.to_string(), SYMBOL_KEY.to_string()],
        )
        .expect("should query symbol property of the contract")
        .into_t()
        .expect("should be string");
    assert_eq!(symbol, "TOK");

    let decimals: u8 = test_context
        .query(
            *DEFAULT_ACCOUNT_ADDR,
            &[CONTRACT_KEY.to_string(), DECIMALS_KEY.to_string()],
        )
        .expect("should query decimals property of the contract")
        .into_t()
        .expect("should be u8");
    assert_eq!(decimals, 255);
}
