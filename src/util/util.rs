use rust_decimal::prelude::*;
use web3::types::U256;

pub fn shorten_address(addr: &String) -> String {
    format!(
        "{}{}{}",
        &addr[..addr.char_indices().nth(4).unwrap().0],
        "...",
        &addr[addr.char_indices().nth_back(4).unwrap().0..]
    )
}

pub fn asset_in_amount(amount: U256, asset: &str) -> Decimal {
    let amount_i128 = amount.as_u128() as i128;
    match asset {
        "USDC" => Decimal::from_i128_with_scale(amount_i128, 6),
        "ETH" => Decimal::from_i128_with_scale(amount_i128, 18),
        "BTC" => Decimal::from_i128_with_scale(amount_i128, 8),
        "DOT" => Decimal::from_i128_with_scale(amount_i128, 10),
        "FLIP" => Decimal::from_i128_with_scale(amount_i128, 18),
        _ => panic!("Unknown asset: {}", asset),
    }
}
fn get_decimals(asset: &str) -> i32 {
    match asset {
        "DOT" => 10,
        "ETH" => 18,
        "BTC" => 8,
        "USDC" => 6,
        "FLIP" => 18,
        _ => panic!("Unknown asset: {}", asset),
    }
}

// (1.0001^tick) * <BASE_ASSET_PRECISION> / <QUOTE_ASSET_PRECISION>.
// BASE_ASSET_PRECISION = 10^scale
pub fn tick_to_price(tick: i32, base_asset: &str, quote_asset: &str) -> f32 {
    1.0001_f32.powi(tick)
        * (10_f32
            .powi(get_decimals(base_asset) - get_decimals(quote_asset))
            .to_f32()
            .unwrap())
}
