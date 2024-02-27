use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::Serialize;
use jsonrpsee::rpc_params;
use poise::serenity_prelude::{self as serenity};
use serenity::Colour;
use serde::Deserialize;
use web3::types::U256;
use crate::{Context, Error};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(bound = "")]
pub struct LimitOrder {
    pub lp: String,
    pub id: U256,
    pub tick: i32,
    pub sell_amount: U256,
    pub fees_earned: U256,
    pub original_sell_amount: U256,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(bound = "")]
pub struct PoolPairsMap {
    pub base: U256,
    pub quote: U256,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(bound = "")]
pub struct Range {
    pub start: i32,
    pub end: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(bound = "")]
pub struct RangeOrder {
    pub lp: String,
    pub id: U256,
    pub range: Range,
    pub liquidity: u128,
    pub fees_earned: PoolPairsMap,
}


#[derive(Deserialize, Clone)]
#[serde(bound = "")]
pub struct AskBidMap {
    pub asks: Vec<LimitOrder>,
    pub bids: Vec<LimitOrder>,
}

#[derive(Clone,  Deserialize)]
pub struct PoolOrders {
    pub limit_orders: AskBidMap,
    pub range_orders: Vec<RangeOrder>,
}

#[poise::command(prefix_command, slash_command, subcommands("orders"), subcommand_required)]
pub async fn lp(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}


/// Lists the top orders for the asset.
#[poise::command(prefix_command, slash_command)]
pub async fn orders(
    ctx: Context<'_>,
    #[description = "Base asset"]
    asset: String,
    #[description = "Quote asset"]
    quote_asset: Option<String>
) -> Result<(), Error> {
    ctx.defer().await?;
    // TODO: Add valid asset check and error handling. Set default quote asset.
    let orders: PoolOrders = ctx.data().http_client.request("cf_pool_orders", rpc_params![asset.to_uppercase(),  quote_asset.unwrap_or("USDC".to_string()) ]).await.expect("request failed");
    let highest_bid = orders.limit_orders.bids.first().unwrap();
    let lowest_ask = orders.limit_orders.asks.first().unwrap();
    ctx.send(poise::CreateReply::default()
        .embed(serenity::CreateEmbed::new()
            .title("Highest Bid")
            .colour(Colour::DARK_GREEN)
            // TODO: use some util to shorten IDs
            .field("LP", format!("{}",highest_bid.lp), true)
            .field("ID", format!("{}",highest_bid.id), true)
            .field("Tick", format!("{}",highest_bid.tick), true)
            // TODO: Add unit conversion
            .field("Sell amount", format!("{}",highest_bid.sell_amount), true)
            .field("Fees earned", format!("{}",highest_bid.fees_earned), true)
        )
        .embed(serenity::CreateEmbed::new()
            .title("Lowest Ask")
            .colour(Colour::DARK_RED)
            .field("LP", format!("{}",lowest_ask.lp), true)
            .field("ID", format!("{}",lowest_ask.id), true)
            .field("Tick", format!("{}",lowest_ask.tick), true)
            .field("Sell amount", format!("{}",lowest_ask.sell_amount), true)
            .field("Fees earned", format!("{}",lowest_ask.fees_earned), true)
        )
        .ephemeral(false)
    ).await?;
    Ok(())
}
