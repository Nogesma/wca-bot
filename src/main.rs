use crate::{
    config::COUNTRIES,
    controllers::{
        Controller,
        competitions::{COMPS, Competitions},
        live::{LIVE, Live},
    },
    jobs::jobs,
};
use dotenv::dotenv;
use log::info;
use serenity::{
    Client,
    all::{ChannelId, CreateForumPost, CreateMessage, EventHandler, GatewayIntents, Ready},
    async_trait,
    prelude::*,
};
use std::{env, fs::create_dir, io, path::PathBuf};
use tokio::sync::Mutex;
use tokio_cron_scheduler::JobScheduler;

mod config;
mod controllers;
mod init;
mod jobs;

const DATA_DIR: &str = "./data";

async fn init_dir() -> Result<(), io::Error> {
    let path = PathBuf::from(DATA_DIR);

    if !path.exists() {
        create_dir(path)?;
        info!("Created data directory.");
    }

    let live = Live::init().await.expect("Unable to init wcalive");
    LIVE.get_or_init(|| Mutex::new(live));

    let comps = Competitions::init()
        .await
        .expect("Unable to init competitions");
    COMPS.get_or_init(|| Mutex::new(comps));

    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Bot ready!");
        init_dir().await.expect("Unable to init dir");

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

    let sched = JobScheduler::new()
        .await
        .expect("Unable to create job scheduler");

    for j in jobs().expect("Unable to create jobs") {
        sched.add(j).await.expect("Unable to add job to scheduler");
    }

    let token = env::var("TOKEN").expect("Missing discord token");

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
