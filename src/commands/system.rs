use std::collections::HashMap;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::Serialize;
use jsonrpsee::rpc_params;
use poise::serenity_prelude::{self as serenity};
use serde::Deserialize;
use serenity::Colour;
use web3::types::{H256, U256, U64};

use time::OffsetDateTime as DateTime;
use time::Duration;

use crate::{Context, Error};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(bound = "")]
#[allow(non_snake_case)]
pub struct BlockHeader {
    pub parentHash: H256,
    pub number: U64,
    pub stateRoot: H256,
    pub extrinsicsRoot: H256,
    #[serde(skip_deserializing)]
    digest: Option<HashMap<String, Vec<String>>>
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(bound = "")]
pub struct AuctionState {
    pub blocks_per_epoch: u32,
    pub current_epoch_started_at: u32,
    pub redemption_period_as_percentage: u8,
    pub min_funding: U256,
    pub auction_size_range: Vec<u16>,
    pub min_active_bid: U256,
}

/// Displays RPC endpoint version
#[poise::command(slash_command, prefix_command)]
pub async fn cf_version(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.defer().await?;
    let response: String = ctx.data().http_client.request("system_version", rpc_params![]).await.expect("request failed");
    ctx.send(poise::CreateReply::default()
        .embed(serenity::CreateEmbed::new()
            .title("System Version")
            .colour(Colour::DARK_GREY)
            .field("Version", response, true)
        )
        .ephemeral(false)
    ).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn cf_auction_state(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.defer().await?;
    let auction: AuctionState = ctx.data().http_client.request("cf_auction_state", rpc_params![]).await.expect("request failed");
    let block_header: BlockHeader = ctx.data().http_client.request("chain_getHeader", rpc_params![]).await.expect("request failed");
    let current_epoch_at: u32 = ctx.data().http_client.request("cf_current_epoch_started_at", rpc_params![]).await.expect("request failed");
    let seconds_to_rotation: u32 = (auction.blocks_per_epoch - (block_header.number.as_u32() - current_epoch_at)) * 6;
    let now = DateTime::now_utc();
    ctx.send(poise::CreateReply::default()
        .embed(serenity::CreateEmbed::new()
            .title("Auction State")
            .colour(Colour::DARK_GREY)
            .field("Blocks per epoch",  format!("{}",auction.blocks_per_epoch)  , true)
            .field("Blocks number", format!("{}",block_header.number), true)
            .field("Next rotation", format!("{}",now + Duration::seconds(seconds_to_rotation as i64)), true)
        )
        .ephemeral(false)
    ).await?;
    Ok(())
}