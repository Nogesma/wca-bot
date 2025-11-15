use crate::init::InitError;
use serde::{Serialize, de::DeserializeOwned};
use serenity::builder::CreateEmbed;
use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufReader, BufWriter},
    mem::swap,
    ops::Deref,
    path::PathBuf,
};

pub mod competitions;
pub mod live;

pub type Result<T> = std::result::Result<T, InitError>;

pub trait ControllerInner: Sized + Serialize + DeserializeOwned {
    const PATH: &str;
    const URL: &str;

    type Inner: Hash + Eq + Clone;

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

    fn format(self) -> impl Iterator<Item = CreateEmbed>;

    async fn download() -> Result<Self>;

    fn new(value: HashSet<Self::Inner>) -> Self;

    fn send(_messages: impl Iterator<Item = CreateEmbed>) -> Result<()> {
        todo!();
        Ok(())
    }
}

pub trait Controller: ControllerInner {
    async fn init() -> Result<Self> {
        if let Ok(s) = Self::read() {
            return Ok(s);
        }

        let s = Self::download().await?;

        s.save()?;

        Ok(s)
    }

    async fn update_and_send(&mut self) -> Result<()>
    where
        Self: Deref<Target = HashSet<Self::Inner>>,
    {
        let diff = self.update().await?;

        let messages = diff.format();

        Self::send(messages)
    }
}
