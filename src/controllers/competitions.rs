use super::{Controller, ControllerInner, EmbedMessage};
use crate::{
    config::{COUNTRIES, EVENT_EMOJI},
    error::{InitError, Result},
};
use chrono::{Datelike, Local};
use futures::StreamExt;
use log::error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serenity::{
    all::{CreateEmbed, EmojiId, GuildChannel, ReactionType},
    model::Color,
};
use std::{collections::HashSet, ops::Deref, sync::OnceLock};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub struct Competition {
    id: String,
    name: String,
    registration_open: String,
    registration_close: String,
    start_date: String,
    end_date: String,
    competitor_limit: Option<u16>,
    cancelled_at: Option<String>,
    url: String,
    city: String,
    venue_address: String,
    latitude_degrees: Value,
    longitude_degrees: Value,
    country_iso2: String,
    event_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Competitions(HashSet<Competition>);

pub static COMPS: OnceLock<Mutex<Controller<Competitions>>> = OnceLock::new();

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

    fn format(self, channels: &'_ [GuildChannel]) -> Vec<EmbedMessage<'_>> {
        self.0.into_iter().map(|comp| {
            let country = COUNTRIES.get(comp.country_iso2.as_str()).unwrap();
            let channel = channels.iter().find(|c| c.name == format!("{} {}", country.0, country.1)).unwrap();
            let thumbnail =
                "https://raw.githubusercontent.com/thewca/worldcubeassociation.org/e974e9020e8c8a1e562c57695b96b312efb8eafa/WcaOnRails/public/files/WCAlogo_50x50.png";
            if comp.cancelled_at.is_some() {
                EmbedMessage {
                    message: CreateEmbed::new().title(format!("**{}**", comp.name)).url(comp.url).thumbnail(thumbnail).colour(Color::new(0xFF0000)).description("La compétition a été annulée."),
                    reactions: vec![
                        ReactionType::Custom {
                            animated: false,
                            id: EmojiId::new(421349840467787776),
                            name: Some("RIP".to_string())
                        }
                        ],
                    channel,
                }
            } else {
                EmbedMessage {
                    message:
                    CreateEmbed::new()
                        .title(format!("**{}**", comp.name))
                        .url(comp.url)
                        .thumbnail(thumbnail).colour(Color::new(0x00FF00)).fields([
                        ("Ville", comp.city, true),
                        ("Pays", format!("__**{}**__ {}", country.1, country.0), true),
                        ("Adresse", format!("[{}](https://duckduckgo.com/?ia=maps&iaxm=maps&q={},{})", comp.venue_address, comp.latitude_degrees, comp.longitude_degrees), false),
                        ("Competiteurs max", comp.competitor_limit.unwrap_or_default().to_string(), false),
                        ("Date", prettify_two_dates(&comp.start_date, &comp.end_date), true),
                        ("Inscriptions", prettify_two_dates(&comp.registration_open, &comp.registration_close), true),
                    ]),
                    reactions: [
                        ReactionType::Custom {
                            animated: false,
                            id: EmojiId::new(862620349376364554),
                            name: Some("WCA".to_string())
                        }
                        ].into_iter().chain(comp.event_ids.iter().map(|id| EVENT_EMOJI.get(id.as_str()).unwrap().clone())).collect(),
                    channel,
                }
            }
        }).collect()
    }

    async fn download(client: &Client) -> Result<Self> {
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
            futures::stream::iter((1..=pages).map(async |page| {
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
                .flat_map(|d| {
                    serde_json::from_str::<Vec<Competition>>(&d)
                        .map_err(|e| error!("Unable to parse competitions: {e}"))
                        .unwrap_or_default()
                })
                .filter(|c| COUNTRIES.contains_key(c.country_iso2.as_str()))
                .collect(),
        ))
    }

    fn new(value: HashSet<Self::Inner>) -> Self {
        Self(value)
    }
}

fn prettify_two_dates(start: &str, end: &str) -> String {
    let start = start.chars().take(10).collect::<String>();
    let end = end.chars().take(10).collect::<String>();

    let start = chrono::NaiveDate::parse_from_str(&start, "%F").unwrap();
    let end = chrono::NaiveDate::parse_from_str(&end, "%F").unwrap();

    let formatted_date = end.format("%d/%m/%Y");

    if start.year() == end.year() {
        if start.month() == end.month() {
            if start.day() == end.day() {
                return formatted_date.to_string();
            }
            return format!("{} au {formatted_date}", start.format("%d"));
        }
        return format!("{} au {formatted_date}", start.format("%d/%m"));
    }
    format!("{} au {formatted_date}", start.format("%d/%m/%Y"))
}
