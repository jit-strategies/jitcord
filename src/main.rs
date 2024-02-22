use poise::serenity_prelude::{self as serenity};
use poise::serenity_prelude::Colour;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


//#[derive(Debug)]
pub struct Data {
    http_client: HttpClient,
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
    .ephemeral(false) // this one only applies in application commands though
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
            commands: vec![age(),cf_version()],
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