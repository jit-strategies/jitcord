use web3::types::U256;
use rust_decimal::prelude::*;

pub fn shorten_address(addr: &String) -> String  {
    format!( "{}{}{}", &addr[..addr.char_indices().nth(4).unwrap().0], "...", &addr[addr.char_indices().nth_back(4).unwrap().0..])
}

pub fn asset_in_amount(amount: U256, asset: &str) -> Decimal {
    let amount_i64 = amount.as_u128().to_i64().unwrap();
    match asset {
        "USDC" => Decimal::new(amount_i64, 6),
        "ETH" => Decimal::new(amount_i64, 18),
        "BTC" => Decimal::new(amount_i64, 8),
        "DOT" => Decimal::new(amount_i64, 10),
        "FLIP" => Decimal::new(amount_i64, 18),
        _ => panic!("Unknown asset: {}", asset),
    }
}