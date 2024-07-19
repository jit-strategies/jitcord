use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::Serialize;
use jsonrpsee::rpc_params;
use poise::serenity_prelude::{self as serenity, CreateEmbed};
use serde::Deserialize;
use serenity::Colour;


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
    subcommands("trigger", "list"),
    subcommand_required
)]
pub async fn pd(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn pd_trigger(
    ctx: Context<'_>,
    #[description = "Summary of the alert"] summary: String,
    #[description = "Severity of the alert"] severity: String,
) -> Result<(), Error> {
    let data = PdData {
        payload: PdPayload {
            summary,
            severity,
            source,
        },
        routing_key: std::env::var("JIDCORD_PAGERDUTY_KEY").unwrap(),
        event_action: "trigger".to_string(),
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://events.eu.pagerduty.com/v2/enqueue")
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?;

    let res_text = res.text().await?;
    let res_json: serde_json::Value = serde_json::from_str(&res_text)?;

    let embed = CreateEmbed::default()
        .title("PagerDuty Alert")
        .field("Summary", summary, false)
        .field("Severity", severity, false)
        .field("Response", res_json, false)
        .colour(Colour::DARK_PURPLE);

    poise::send_reply(ctx, |f| f.set_embed(embed)).await?;

    Ok(())
}