use crate::init::{InitError, Result};
use serde::{de::DeserializeOwned, Serialize};
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
    http: Arc<Http>,
    channels: Vec<GuildChannel>,
    data: T,
}

impl<T: ControllerInner> Controller<T> {
    pub async fn init(http: Arc<Http>, channels: Vec<GuildChannel>) -> Result<Self> {
        Ok(Self {
            http,
            channels,
            data: T::init().await?,
        })
    }

    pub async fn update_and_send(&mut self) -> Result<()>
    where
        T: Deref<Target = HashSet<T::Inner>>,
    {
        self.data.update_and_send(&self.http, &self.channels).await
    }
}

pub trait ControllerInner: Sized + Serialize + DeserializeOwned + Send {
    const PATH: &str;
    const URL: &str;

    type Inner: Hash + Eq + Clone;

    async fn init() -> Result<Self> {
        if let Ok(s) = Self::read() {
            return Ok(s);
        }

        let s = Self::download().await?;

        s.save()?;

        Ok(s)
    }

    async fn update_and_send(
        &mut self,
        http: impl CacheHttp,
        channels: &[GuildChannel],
    ) -> Result<()>
    where
        Self: Deref<Target = HashSet<Self::Inner>>,
    {
        let diff = self.update().await?;

        let messages = diff.format(channels);

        Self::send(http, messages).await
    }

    fn read() -> Result<Self> {
        let path = PathBuf::from(Self::PATH);

        let reader = BufReader::new(File::open(path)?);

        Ok(serde_json::from_reader(reader)?)
    }

    async fn update(&mut self) -> Result<Self>
    where
        Self: Deref<Target = HashSet<Self::Inner>>,
    {
        let mut new = Self::download().await?;

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

        let writer = BufWriter::new(File::open(path)?);

        Ok(serde_json::to_writer(writer, self)?)
    }

    fn format(self, channels: &'_ [GuildChannel]) -> Vec<EmbedMessage<'_>>;

    async fn download() -> Result<Self>;

    fn new(value: HashSet<Self::Inner>) -> Self;

    // We cannot use an impl Iterator instead of a Vec here.
    // (see issue #100013 <https://github.com/rust-lang/rust/issues/100013> for more information)
    async fn send(http: impl CacheHttp, messages: Vec<EmbedMessage<'_>>) -> Result<()> {
        for m in messages.into_iter() {
            let mess = CreateMessage::new()
                .add_embed(m.message)
                .reactions(m.reactions);

            m.channel
                .send_message(&http, mess)
                .await
                .map_err(|_| InitError::Serenity)?;
        }

        Ok(())
    }
}
