use poise::serenity_prelude::{self as serenity};
use poise::serenity_prelude::Colour;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// use primitive_types::U256;

use web3::types::U256;
use serde::{Serialize,Deserialize};

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
//#[derive(Debug)]
pub struct Data {
    http_client: HttpClient,
}

#[derive(Clone,  Deserialize)]
pub struct PoolOrders {
	pub limit_orders: AskBidMap,
	pub range_orders: Vec<RangeOrder>,
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Displays RPC endpoint version
#[poise::command(slash_command, prefix_command)]
async fn cf_version(
    ctx: Context<'_>
) -> Result<(), Error> {
    //ctx.defer().await?;

    let response: String = ctx.data().http_client.request("system_version", rpc_params![]).await.expect("request failed");
    //ctx.say(response).await?;
    ctx.send(poise::CreateReply::default()
    .content("Works for slash and prefix commands")
    .embed(serenity::CreateEmbed::new()
        .title("Much versatile, very wow")
        .colour(Colour::DARK_GREEN)
        .field("Version", response, true)
        .url("http://jitstrategies.com")
    )
    .ephemeral(false)
    ).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn cf_pool_orders(
    ctx: Context<'_>
) -> Result<(), Error> {
    //ctx.defer().await?;
    let orders: PoolOrders = ctx.data().http_client.request("cf_pool_orders", rpc_params!["ETH", "USDC"]).await.expect("request failed");
    let highest_bid = orders.limit_orders.bids.first().unwrap();
    let lowest_ask = orders.limit_orders.asks.first().unwrap();
    ctx.send(poise::CreateReply::default()
    .embed(serenity::CreateEmbed::new()
        .title("Highest Bid")
        .colour(Colour::DARK_GREEN)
        .field("LP", format!("{}",highest_bid.lp), true)
        .field("ID", format!("{}",highest_bid.id), true)
        .field("Tick", format!("{}",highest_bid.tick), true)
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


#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = std::env::var("JITCORD_DISCORD_TOKEN").expect("missing JITCORD_DISCORD_TOKEN env var!");
    let target = std::env::var("JITCORD_TARGET").expect("missing JITCORD_TARGET env var!");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(),cf_version(),cf_pool_orders()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let client = HttpClientBuilder::default().build(target).unwrap();
                Ok(Data {http_client: client})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
    Ok(())
}