use casper_types::U512;

use crate::erc20::{token_cfg, Sender, Token};

#[test]
fn test_erc20_deploy() {
    let t = Token::deploy();
    assert_eq!(t.name(), token_cfg::NAME);
    assert_eq!(t.symbol(), token_cfg::SYMBOL);
    assert_eq!(t.decimals(), token_cfg::DECIMALS);
    assert_eq!(t.balance_of(t.ali), Some(token_cfg::total_supply()));
}

#[test]
fn test_erc20_transfer() {
    let transfer_amount_1 = U512::from(42);
    let transfer_amount_2 = U512::from(20);

    let mut t = Token::deploy();
    // ali -> bob
    assert_eq!(t.balance_of(t.bob), None);
    assert_eq!(t.balance_of(t.ali), Some(token_cfg::total_supply()));
    t.transfer(t.bob, transfer_amount_1, Sender(t.ali));
    assert_eq!(t.balance_of(t.bob), Some(transfer_amount_1));
    assert_eq!(
        t.balance_of(t.ali),
        Some(token_cfg::total_supply() - transfer_amount_1)
    );

    // bob -> ali

    t.transfer(t.ali, transfer_amount_2, Sender(t.bob));
    assert_eq!(
        t.balance_of(t.ali),
        Some(token_cfg::total_supply() - transfer_amount_1 + transfer_amount_2),
    );
    assert_eq!(
        t.balance_of(t.bob),
        Some(transfer_amount_1 - transfer_amount_2)
    );
}

#[test]
#[should_panic(expected = "ApiError::User(1) [65537]")]
fn test_should_not_approve_more_than_owners_balance() {
    // NOTE: exercises the happy path

    let approve_amount = U512::from(100);
    let transfer_amount = U512::from(42);
    assert!(approve_amount > transfer_amount);

    let mut t = Token::deploy();

    let owner = t.ali;
    let spender = t.joe;

    let owners_balance = t.balance_of(owner).expect("owner should have balance");

    t.approve(spender, owners_balance + U512::one(), Sender(owner));
}

#[test]
fn test_erc20_transfer_from() {
    // NOTE: exercises the happy path
    let approve_amount = U512::from(100);
    let transfer_amount = U512::from(42);
    assert!(approve_amount > transfer_amount);

    let mut t = Token::deploy();

    let owner = t.ali;
    let spender = t.bob;
    let recipient = t.joe;

    let owner_balance_before = t.balance_of(owner).expect("owner should have balance");
    t.approve(spender, approve_amount, Sender(owner));
    assert_eq!(t.allowance(owner, spender), Some(approve_amount));

    t.transfer_from(owner, recipient, transfer_amount, Sender(spender));

    assert_eq!(
        t.balance_of(owner),
        Some(owner_balance_before - transfer_amount),
        "should decrease balance of the owner"
    );
    assert_eq!(
        t.allowance(owner, spender),
        Some(approve_amount - transfer_amount),
        "should decrease allowance of the spender"
    );
    assert_eq!(
        t.balance_of(recipient),
        Some(transfer_amount),
        "recipient should receive tokens"
    );
}
