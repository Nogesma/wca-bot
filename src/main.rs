use country_emoji;
use dotenv::dotenv;
use log::info;
use serenity::{
    Client,
    all::{
        ChannelId, CreateForumPost, CreateMessage, EventHandler, GatewayIntents, GetMessages,
        Guild, Ready,
    },
    async_trait,
    http::Http,
    prelude::*,
};
use std::{env, fs::create_dir, io, path::PathBuf};

mod config;

use config::COUNTRIES;

const DATA_DIR: &str = "./data";

fn get_recent_records() -> Result<(), io::Error> {
    Ok(())
}
fn update_wca_live(_: ()) -> Result<(), io::Error> {
    Ok(())
}
fn get_upcoming_competitions() -> Result<(), io::Error> {
    Ok(())
}
fn update_wca_comps(_: ()) -> Result<(), io::Error> {
    Ok(())
}
fn get_competitions() -> Result<(), io::Error> {
    Ok(())
}

fn init_dir() -> Result<(), io::Error> {
    let path = PathBuf::from(DATA_DIR);

    if !path.exists() {
        create_dir(path)?;
        info!("Created data directory.");
    }

    let wcalive = PathBuf::from_iter([DATA_DIR, "wcalive.json"]);
    if !wcalive.exists() {
        let records = get_recent_records()?;
        update_wca_live(records)?;
        info!("Updated WCA live data.");
    }

    let wcacomps = PathBuf::from_iter([DATA_DIR, "wcacomps.json"]);
    if !wcacomps.exists() {
        let upcoming_competitions = get_upcoming_competitions()?;
        update_wca_comps(upcoming_competitions)?;
        info!("Updated WCA comps data.");
    }

    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Bot ready!");
        init_dir().expect("Unable to init dir");

        // Expect the bot to only be in one guild
        let guild_id = ready.guilds.first().unwrap().id;

        let channels = guild_id
            .channels(&ctx.http)
            .await
            .expect("Unable to get guild channels.");

        let forum = channels
            .get(&ChannelId::new(
                env::var("WCA_COMP").unwrap().parse().unwrap(),
            ))
            .unwrap();

        eprintln!("{forum:?}");

        let active_threads = guild_id.get_active_threads(&ctx.http).await.unwrap();

        let active_threads = active_threads
            .threads
            .into_iter()
            .filter(|t| {
                t.parent_id
                    == Some(ChannelId::new(
                        env::var("WCA_COMP").unwrap().parse().unwrap(),
                    ))
            })
            .collect::<Vec<_>>();

        let mut archived_threads = forum
            .id
            .get_archived_public_threads(&ctx.http, None, None)
            .await
            .unwrap()
            .threads;

        let mut threads = active_threads;
        threads.append(&mut archived_threads);

        for (flag, name) in COUNTRIES.values() {
            let channel_name = format!("{flag} {name}",);

            if threads.iter().any(|t| t.name == channel_name) {
                continue;
            }

            let thread_builder = CreateForumPost::new(
                channel_name,
                CreateMessage::new().content(format!("Compétitions officielles: **{name}**")),
            );

            forum
                .create_forum_post(&ctx.http, thread_builder)
                .await
                .unwrap();

            info!("Created channel for {name}.");
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TOKEN").expect("Missing discord token.");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_EMOJIS_AND_STICKERS
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create discord client.");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }
}
