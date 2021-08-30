use crate::erc20::{token_cfg, Token};

#[test]
fn test_erc20_deploy() {
    let t = Token::deploy();
    assert_eq!(t.name(), token_cfg::NAME);
    assert_eq!(t.symbol(), token_cfg::SYMBOL);
    assert_eq!(t.decimals(), token_cfg::DECIMALS);
    assert_eq!(t.balance_of(t.ali), token_cfg::total_supply());
}