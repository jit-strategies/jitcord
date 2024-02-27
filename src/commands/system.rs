use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use poise::serenity_prelude::{self as serenity};
use serenity::Colour;
use crate::{Context, Error};

/// Displays RPC endpoint version
#[poise::command(slash_command, prefix_command)]
pub async fn cf_version(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.defer().await?;
    let response: String = ctx.data().http_client.request("system_version", rpc_params![]).await.expect("request failed");
    ctx.send(poise::CreateReply::default()
        .embed(serenity::CreateEmbed::new()
            .title("Version")
            .colour(Colour::DARK_GREEN)
            .field("Version", response, true)
        )
        .ephemeral(false)
    ).await?;
    Ok(())
}