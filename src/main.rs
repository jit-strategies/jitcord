use poise::serenity_prelude as serenity;

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
    let response: String = ctx.data().http_client.request("system_version", rpc_params![]).await.expect("request failed");
    ctx.say(response).await?;
    Ok(())
}


#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(),cf_version()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let client = HttpClientBuilder::default().build("https://cf-berghain.jitstrategies.xyz").unwrap();
                Ok(Data {http_client: client})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}