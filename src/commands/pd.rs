use jsonrpsee::core::Serialize;
use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbed};
use serde::Deserialize;
use serenity::Colour;
use futures::{Stream, StreamExt};

// Example call
// curl --request 'POST' \
// --url 'https://events.eu.pagerduty.com/v2/enqueue' \
// --header 'Content-Type: application/json' \
// --data '{
//   "payload": {
//       "summary": "Test alert",
//       "severity": "critical",
//       "source": "Alert source"
//   },
//   "routing_key": "R0...GXR",
//   "event_action": "trigger"
// }'

#[derive(Serialize, Deserialize, Clone)]
#[serde(bound = "")]
pub struct PdPayload {
    pub summary: String,
    pub severity: String,
    pub source: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(bound = "")]
pub struct PdData {
    pub payload: PdPayload,
    pub routing_key: String,
    pub event_action: String,
}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("trigger"),
    subcommand_required
)]
pub async fn pd(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn trigger(
    ctx: Context<'_>,
    #[description = "Summary of the alert"] summary: String,
    #[description = "Severity of the alert"]
    #[autocomplete = "autocomplete_severity"]
    severity: String,
) -> Result<(), Error> {
    let source = "JITCORD".to_string();
    let data = PdData {
        payload: PdPayload {
            summary,
            severity,
            source,
        },
        routing_key: std::env::var("JITCORD_PAGERDUTY_KEY").unwrap(),
        event_action: "trigger".to_string(),
    };
    std::env::var("JITCORD_PAGERDUTY_KEY").expect("missing JITCORD_PAGERDUTY_KEY env var!");
    let client = reqwest::Client::new();
    let res = client
        .post("https://events.eu.pagerduty.com/v2/enqueue")
        .json(&data)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let res_text = res.text().await?;
    let embed = CreateEmbed::default()
        .title("PagerDuty Alert")
        .field("Summary", data.payload.summary, false)
        .field("Severity", data.payload.severity, false)
        .field("Response", res_text, false)
        .colour(Colour::DARK_PURPLE);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

async fn autocomplete_severity<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&["critical", "warning", "error", "info"])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}