mod commands;
mod util;

use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use poise::serenity_prelude::{self as serenity};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug)]
pub struct Data {
    http_client: HttpClient,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token =
        std::env::var("JITCORD_DISCORD_TOKEN").expect("missing JITCORD_DISCORD_TOKEN env var!");
    let target = std::env::var("JITCORD_TARGET").expect("missing JITCORD_TARGET env var!");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::system::cf_version(),
                commands::lp::lp(),
                commands::system::cf_auction_state(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let client = HttpClientBuilder::default().build(target).unwrap();
                Ok(Data {
                    http_client: client,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
    Ok(())
}
