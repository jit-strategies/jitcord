use crate::util::util::{asset_in_amount, bool_to_emoji};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::Serialize;
use jsonrpsee::rpc_params;
use poise::serenity_prelude::{self as serenity, CreateEmbed};
use serde::Deserialize;
use serenity::Colour;
use std::collections::{BTreeMap, HashMap};
use tap::pipe::Pipe;
use web3::types::{Address, H256, U256, U64};

use time::OffsetDateTime as DateTime;
use time::{format_description, Duration};

use crate::{Context, Error};

const DATE_FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(bound = "")]
#[allow(non_snake_case)]
pub struct BlockHeader {
    pub parentHash: H256,
    pub number: U64,
    pub stateRoot: H256,
    pub extrinsicsRoot: H256,
    #[serde(skip_deserializing)]
    digest: Option<HashMap<String, Vec<String>>>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(bound = "")]
#[allow(non_snake_case)]
pub struct SystemHealth {
    pub peers: u32,
    pub isSyncing: bool,
    pub shouldHavePeers: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(bound = "")]
pub struct AccountList(Vec<AccountPair>);

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(bound = "")]
struct AccountPair(String, String);

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(bound = "")]
pub struct AccountInfoV2 {
    balance: U256,
    bond: U256,
    last_heartbeat: u32,
    reputation_points: u16,
    keyholder_epochs: Vec<u64>,
    is_current_authority: bool,
    is_current_backup: bool,
    is_qualified: bool,
    is_online: bool,
    is_bidding: bool,
    bound_redeem_address: Option<Address>,
    apy_bp: Option<u32>,
    #[serde(skip_deserializing)]
    restricted_balances: Option<HashMap<Address, U256>>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum AccountInfo {
    Unregistered {
        flip_balance: U256,
    },
    Broker {
        flip_balance: U256,
    },
    LiquidityProvider {
        balances: HashMap<String, HashMap<String, U256>>,
        refund_addresses: HashMap<String, Option<String>>,
        flip_balance: U256,
    },
    Validator {
        flip_balance: U256,
        bond: U256,
        last_heartbeat: u32,
        reputation_points: i32,
        keyholder_epochs: Vec<u32>,
        is_current_authority: bool,
        is_current_backup: bool,
        is_qualified: bool,
        is_online: bool,
        is_bidding: bool,
        bound_redeem_address: Option<Address>,
        apy_bp: Option<u32>,
        restricted_balances: BTreeMap<Address, U256>,
    },
}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("status", "auction", "account_info"),
    subcommand_required
)]
pub async fn cf(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Displays Chainflip endpoint status
#[poise::command(slash_command, prefix_command)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let version: String = ctx
        .data()
        .http_client
        .request("system_version", rpc_params![])
        .await
        .expect("request failed");
    let health: SystemHealth = ctx
        .data()
        .http_client
        .request("system_health", rpc_params![])
        .await
        .expect("request failed");
    ctx.send(
        poise::CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .title("System Status")
                    .colour(Colour::DARK_GREY)
                    .field("Version", version, true)
                    .field("Peers", format!("{}", health.peers), true)
                    .field("Synced", format!("{}", !health.isSyncing), true),
            )
            .ephemeral(false),
    )
    .await?;
    Ok(())
}

/// Displays auction related data
#[poise::command(slash_command, prefix_command)]
pub async fn auction(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let date_format = format_description::parse(DATE_FORMAT)?;
    let auction: AuctionState = ctx
        .data()
        .http_client
        .request("cf_auction_state", rpc_params![])
        .await
        .expect("request failed");
    let block_header: BlockHeader = ctx
        .data()
        .http_client
        .request("chain_getHeader", rpc_params![])
        .await
        .expect("request failed");
    let current_epoch_at: u32 = ctx
        .data()
        .http_client
        .request("cf_current_epoch_started_at", rpc_params![])
        .await
        .expect("request failed");
    let current_epoch: u32 = ctx
        .data()
        .http_client
        .request("cf_current_epoch", rpc_params![])
        .await
        .expect("request failed");
    let seconds_to_rotation: u32 =
        (auction.blocks_per_epoch - (block_header.number.as_u32() - current_epoch_at)) * 6;
    let now = DateTime::now_utc();
    ctx.send(
        poise::CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .title("Auction State")
                    .colour(Colour::DARK_GREY)
                    .field(
                        "Min. Active Bid",
                        format!(
                            "{}",
                            asset_in_amount(&auction.min_active_bid, "FLIP").round_dp(3)
                        ),
                        true,
                    )
                    .field("Current block", format!("{}", block_header.number), true)
                    .field("Current epoch", format!("{}", current_epoch), true)
                    .field(
                        "Next rotation",
                        format!(
                            "{} UTC",
                            (now + Duration::seconds(seconds_to_rotation as i64))
                                .format(&date_format)
                                .unwrap()
                        ),
                        true,
                    ),
            )
            .ephemeral(false),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn account_info(
    ctx: Context<'_>,
    #[description = "Account name or address"] name: String,
) -> Result<(), Error> {
    let accounts: AccountList = ctx
        .data()
        .http_client
        .request("cf_accounts", rpc_params![])
        .await
        .expect("request failed");
    match search_account_by_name(&accounts, name) {
        Some(acc) => {
            let account_info: AccountInfo = ctx
                .data()
                .http_client
                .request("cf_account_info", rpc_params![&acc.0])
                .await
                .expect("request failed");
            match account_info {
                AccountInfo::LiquidityProvider {
                    balances,
                    flip_balance,
                    ..
                } => {
                    ctx.send(
                        poise::CreateReply::default()
                            .embed(
                                CreateEmbed::new()
                                    .title("Liquidity Provider")
                                    .colour(Colour::GOLD)
                                    .field("Account", format!("{}", &acc.0), false)
                                    //.field("Vanity Name", format!("{}", &acc.1), true)
                                    .field(
                                        "Liquidity Balances",
                                        balance_map_format(&balances),
                                        true,
                                    )
                                    .field(
                                        "Account Balance (FLIP)",
                                        format!(
                                            "{}",
                                            asset_in_amount(&flip_balance, "FLIP").round_dp(4)
                                        ),
                                        true,
                                    ),
                            )
                            .ephemeral(false),
                    )
                    .await?;
                }
                AccountInfo::Validator {
                    flip_balance,
                    reputation_points,
                    bound_redeem_address,
                    is_online,
                    is_bidding,
                    is_current_authority,
                    is_qualified,
                    is_current_backup,
                    ..
                } => {
                    ctx.send(
                        poise::CreateReply::default()
                            .embed(
                                CreateEmbed::new()
                                    .title("Validator")
                                    .colour(Colour::DARK_GREY)
                                    .field("Account", format!("{}", &acc.0), false)
                                    .field("Vanity Name", format!("{}", &acc.1), true)
                                    .field(
                                        "Balance",
                                        format!(
                                            "{}",
                                            asset_in_amount(&flip_balance, "FLIP").round_dp(4)
                                        ),
                                        true,
                                    )
                                    .field("Reputation", format!("{}", &reputation_points), true)
                                    .pipe(|it| {
                                        if bound_redeem_address.is_some() {
                                            it.field(
                                                "Bound Redeem Address",
                                                format!("{}", &bound_redeem_address.unwrap()),
                                                true,
                                            )
                                        } else {
                                            it
                                        }
                                    })
                                    .field("Online", format!("{}", bool_to_emoji(is_online)), true)
                                    .field(
                                        "Bidding",
                                        format!("{}", bool_to_emoji(is_bidding)),
                                        true,
                                    )
                                    .field(
                                        "Authority",
                                        format!("{}", bool_to_emoji(is_current_authority)),
                                        true,
                                    )
                                    .field(
                                        "Qualified",
                                        format!("{}", bool_to_emoji(is_qualified)),
                                        true,
                                    )
                                    .field(
                                        "Backup",
                                        format!("{}", bool_to_emoji(is_current_backup)),
                                        true,
                                    ),
                            )
                            .ephemeral(false),
                    )
                    .await?;
                }
                _ => {}
            }
        }
        None => {
            poise::say_reply(ctx, "Account or vanity name not found").await?;
        }
    };
    Ok(())
}

fn search_account_by_name(accs: &AccountList, name: String) -> Option<AccountPair> {
    let mut accounts = accs.clone();
    accounts
        .0
        .retain(|x| x.0.contains(name.as_str()) || x.1.contains(name.as_str()));
    match accounts.0.len() {
        len if len > 0 => Some(accounts.0.pop().unwrap()),
        _ => None,
    }
}

fn balance_map_format(balances: &HashMap<String, HashMap<String, U256>>) -> String {
    let mut balances_formatted = String::from("");
    for (key, val) in &*balances {
        balances_formatted.push_str(format!("{}\n", key).as_str());
        for (ikey, ival) in val {
            balances_formatted.push_str(
                format!("{}: {}\n", ikey, asset_in_amount(ival, ikey).round_dp(4)).as_str(),
            );
        }
    }
    balances_formatted
}
