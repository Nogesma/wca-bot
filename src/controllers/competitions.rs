use super::{Controller, ControllerInner, Result};
use crate::{config::COUNTRIES, init::InitError};
use chrono::Local;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serenity::{all::CreateEmbed, model::Color};
use std::{collections::HashSet, ops::Deref, sync::OnceLock};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct Competition {
    pub id: String,
    pub name: String,
    pub registration_open: String,
    pub registration_close: String,
    pub start_date: String,
    pub end_date: String,
    pub competitor_limit: u16,
    pub cancelled_at: Option<String>,
    pub url: String,
    pub city: String,
    pub venue_address: String,
    pub latitude_degrees: String,
    pub longitude_degrees: String,
    pub country_iso2: String,
    pub events_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Competitions(HashSet<Competition>);

pub static COMPS: OnceLock<Mutex<Competitions>> = OnceLock::new();

impl Deref for Competitions {
    type Target = HashSet<Competition>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ControllerInner for Competitions {
    const PATH: &str = "./data/wcacomps.json";
    const URL: &str =
        "https://www.worldcubeassociation.org/api/v0/competitions?sort=start_date&start=";

    type Inner = Competition;

    async fn download() -> Result<Self> {
        let client = Client::builder()
            .user_agent("Nogesma/wca-bot/2.0")
            .build()?;

        let url = format!("{}{}", Self::URL, Local::now().format("%F"));

        let res = client.get(&url).send().await?.error_for_status()?;

        let headers = res.headers();
        let pages = headers
            .get("Total")
            .ok_or(InitError::MissingHeader)?
            .to_str()?
            .parse::<u32>()?
            / headers
                .get("Per-page")
                .ok_or(InitError::MissingHeader)?
                .to_str()?
                .parse::<u32>()?;

        let mut data = vec![res.text().await?];
        data.extend(
            futures::stream::iter((1..pages).map(async |page| {
                client
                    .get(format!("{url}&page={page}"))
                    .send()
                    .await?
                    .error_for_status()?
                    .text()
                    .await
            }))
            .buffer_unordered(10)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten(),
        );

        Ok(Self(
            data.into_iter()
                .flat_map(|d| serde_json::from_str::<Vec<Competition>>(&d).unwrap())
                .filter(|c| COUNTRIES.contains_key(c.country_iso2.as_str()))
                .collect(),
        ))
    }

    fn new(value: HashSet<Self::Inner>) -> Self {
        Self(value)
    }

    fn format(self) -> impl Iterator<Item = CreateEmbed> {
        self.0.into_iter().map(|comp| {
            CreateEmbed::new()
                .title(format!("**{}**", comp.name))
                .url(comp.url)
                .thumbnail(
                    "https://raw.githubusercontent.com/thewca/worldcubeassociation.org/e974e9020e8c8a1e562c57695b96b312efb8eafa/WcaOnRails/public/files/WCAlogo_50x50.png",
                ).colour(Color::new(0x00FF00)).fields([
                ("Ville", comp.city, true),
                    ("Pays", format!("__**{}**__ {}", COUNTRIES.get(comp.country_iso2.as_str()).unwrap().1, COUNTRIES.get(comp.country_iso2.as_str()).unwrap().0), true),
                ("Adresse", format!("[{}](https://duckduckgo.com/?ia=maps&iaxm=maps&q={},{})", comp.venue_address, comp.latitude_degrees, comp.longitude_degrees), false),
                ("Competiteurs max", comp.competitor_limit.to_string(), false),
                ("Date",  prettify_two_dates(comp.start_date, comp.end_date), true),
                ("Inscriptions",  prettify_two_dates(comp.registration_open, comp.registration_close), true),
            ])
            // TODO: add reactions and send to the right channel
        })
    }
}

impl Controller for Competitions {}
