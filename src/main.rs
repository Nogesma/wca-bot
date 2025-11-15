use crate::{
    config::COUNTRIES,
    controllers::{Controller, competitions::COMPS, live::LIVE},
    init::{InitError, Result},
    jobs::jobs,
};
use dotenv::dotenv;
use log::info;
use serenity::{
    Client,
    all::{
        ChannelId, CreateForumPost, CreateMessage, EventHandler, GatewayIntents, GuildId, Ready,
    },
    async_trait,
    prelude::*,
};
use std::{env, fs::create_dir, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use tokio_cron_scheduler::JobScheduler;

mod config;
mod controllers;
mod init;
mod jobs;

const DATA_DIR: &str = "./data";

async fn init(ctx: Context, guild: GuildId) -> Result<()> {
    let channels = guild
        .channels(&ctx.http)
        .await
        .map_err(|_| InitError::Serenity)?;

    let wca_comps_channel_id = ChannelId::new(env::var("WCA_COMP")?.parse()?);

    let forum = channels
        .get(&wca_comps_channel_id)
        .ok_or(InitError::MissingChannel)?;

    let active_threads = guild
        .get_active_threads(&ctx.http)
        .await
        .map_err(|_| InitError::Serenity)?;

    let active_threads = active_threads
        .threads
        .into_iter()
        .filter(|t| t.parent_id == Some(wca_comps_channel_id))
        .collect::<Vec<_>>();

    let mut archived_threads = forum
        .id
        .get_archived_public_threads(&ctx.http, None, None)
        .await
        .map_err(|_| InitError::Serenity)?
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

        let thread = forum
            .create_forum_post(&ctx.http, thread_builder)
            .await
            .map_err(|_| InitError::Serenity)?;

        threads.push(thread);

        info!("Created channel for {name}.");
    }

    {
        let path = PathBuf::from(DATA_DIR);

        if !path.exists() {
            create_dir(path)?;
            info!("Created data directory.");
        }
    }

    {
        let live_channel = channels
            .get(&ChannelId::new(env::var("WCA_LIVE")?.parse()?))
            .ok_or(InitError::MissingChannel)?
            .clone();

        let live = Controller::init(Arc::clone(&ctx.http), vec![live_channel]).await?;
        LIVE.get_or_init(|| Mutex::new(live));
    }

    {
        let comps = Controller::init(ctx.http, threads).await?;

        COMPS.get_or_init(|| Mutex::new(comps));
    }

    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Bot ready!");

        // Expect the bot to only be in one guild
        let guild_id = ready.guilds.first().unwrap().id;

        init(ctx, guild_id).await.unwrap();
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
