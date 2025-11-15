use crate::error::Result;
use log::{debug, error, info};
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};
use serenity::{
    all::{GuildChannel, ReactionType},
    builder::{CreateEmbed, CreateMessage},
    http::{CacheHttp, Http},
};
use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufReader, BufWriter},
    mem::swap,
    ops::Deref,
    path::PathBuf,
    sync::Arc,
};

pub mod competitions;
pub mod live;

pub struct EmbedMessage<'a> {
    message: CreateEmbed,
    channel: &'a GuildChannel,
    reactions: Vec<ReactionType>,
}

pub struct Controller<T: ControllerInner> {
    http: Client,
    cache: Arc<Http>,
    channels: Vec<GuildChannel>,
    data: T,
}

impl<T: ControllerInner> Controller<T> {
    pub async fn init(http: Client, cache: Arc<Http>, channels: Vec<GuildChannel>) -> Result<Self> {
        let data = T::init(&http).await?;
        Ok(Self {
            http,
            cache,
            channels,
            data,
        })
    }

    pub async fn update_and_send(&mut self) -> Result<()>
    where
        T: Deref<Target = HashSet<T::Inner>>,
    {
        self.data
            .update_and_send(&self.http, &self.cache, &self.channels)
            .await
    }
}

pub trait ControllerInner: Sized + Serialize + DeserializeOwned + Send {
    const PATH: &str;
    const URL: &str;

    type Inner: Hash + Eq + Clone;

    async fn init(http: &Client) -> Result<Self> {
        if let Ok(s) = Self::read() {
            info!("Read data from disk");
            return Ok(s);
        }

        let s = Self::download(http).await?;

        info!("Downloaded initial data from server");

        s.save()?;

        info!("Saved data to file");

        Ok(s)
    }

    async fn update_and_send(
        &mut self,
        http: &Client,
        cache: impl CacheHttp,
        channels: &[GuildChannel],
    ) -> Result<()>
    where
        Self: Deref<Target = HashSet<Self::Inner>>,
    {
        let diff = self.update(http).await?;

        debug!("Successfully ran update");

        let messages = diff.format(channels);

        Self::send(cache, messages).await;

        Ok(())
    }

    fn read() -> Result<Self> {
        let path = PathBuf::from(Self::PATH);

        let reader = BufReader::new(File::open(path)?);

        Ok(serde_json::from_reader(reader)?)
    }

    async fn update(&mut self, http: &Client) -> Result<Self>
    where
        Self: Deref<Target = HashSet<Self::Inner>>,
    {
        let mut new = Self::download(http).await?;

        let diff = Self::new(
            self.symmetric_difference(&*new)
                .cloned()
                .collect::<HashSet<_>>(),
        );

        new.save()?;

        swap(self, &mut new);

        Ok(diff)
    }

    fn save(&self) -> Result<()> {
        let path = PathBuf::from(Self::PATH);

        let writer = BufWriter::new(File::create(path)?);

        Ok(serde_json::to_writer(writer, self)?)
    }

    fn format(self, channels: &'_ [GuildChannel]) -> Vec<EmbedMessage<'_>>;

    async fn download(http: &Client) -> Result<Self>;

    fn new(value: HashSet<Self::Inner>) -> Self;

    // We cannot use an impl Iterator instead of a Vec here.
    // (see issue #100013 <https://github.com/rust-lang/rust/issues/100013> for more information)
    async fn send(http: impl CacheHttp, messages: Vec<EmbedMessage<'_>>) {
        for m in messages.into_iter() {
            let mess = CreateMessage::new()
                .add_embed(m.message)
                .reactions(m.reactions);

            if let Err(e) = m.channel.send_message(&http, mess.clone()).await {
                error!("Serenity error: {e:?}");
            }
        }
    }
}
