use super::{Controller, ControllerInner, EmbedMessage};
use crate::{
    config::{COUNTRIES, EVENT_EMOJI, TAG_COLOR},
    init::{InitError, Result},
};
use cynic::{QueryBuilder, http::ReqwestExt};
use gql::{Record, RootQueryType};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serenity::all::{Color, CreateEmbed, GuildChannel};
use std::{collections::HashSet, ops::Deref, sync::OnceLock};
use tokio::sync::Mutex;

mod gql;

#[derive(Serialize, Deserialize)]
pub struct Live(HashSet<Record>);

pub static LIVE: OnceLock<Mutex<Controller<Live>>> = OnceLock::new();

impl Deref for Live {
    type Target = HashSet<Record>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ControllerInner for Live {
    const PATH: &str = "./data/wcalive.json";
    const URL: &str = "https://live.worldcubeassociation.org/api/graphql";

    type Inner = Record;

    fn format(self, channels: &'_ [GuildChannel]) -> Vec<EmbedMessage<'_>> {
        let channel = channels.first().unwrap();
        self.0
            .into_iter()
            .map(move |record| {
                let event_id = record.result.round.competition_event.event.id.inner();
                EmbedMessage {
                    message: CreateEmbed::new()
                        .title(format!(
                            "{} {} {} of {}",
                            record.result.round.competition_event.event.name,
                            EVENT_EMOJI.get(event_id).unwrap(),
                            get_result_type(&record.t, event_id),
                            format_attempt_result(record.attempt_result, event_id)
                        ))
                        .url(format!(
                            "https://live.worldcubeassociation.org/competitions/{}/rounds/{}",
                            record.result.round.competition_event.competition.id.inner(),
                            record.result.round.id.inner()
                        ))
                        .description(
                            // TODO: Country iso2 will fail, need to import crate
                            format!(
                                "{} from {} {}",
                                record.result.person.name.unwrap_or_default(),
                                record.result.person.country.name,
                                COUNTRIES
                                    .get(record.result.person.country.iso2.as_str())
                                    .unwrap_or(&("", ""))
                                    .0
                            ),
                        )
                        .colour(
                            *TAG_COLOR
                                .get(record.tag.as_str())
                                .unwrap_or(&Color::from(0xFFFFFF)),
                        )
                        .thumbnail(format!(
                            "https://raw.githubusercontent.com/Nogesma/wca-bot/main/img/{}.png",
                            record.tag
                        )),
                    reactions: vec![],
                    channel,
                }
            })
            .collect()
    }

    async fn download() -> Result<Self> {
        let client = Client::builder()
            .user_agent("Nogesma/wca-bot/2.0")
            .build()?;

        let operation = RootQueryType::build(());

        let data = client.post(Self::URL).run_graphql(operation).await?;

        let data = data.data.ok_or(InitError::MissingData)?;

        Ok(Self(HashSet::from_iter(data.recent_records.into_iter())))
    }

    fn new(value: HashSet<Self::Inner>) -> Self {
        Self(value)
    }
}

fn get_result_type<'a>(t: &'a str, event: &str) -> &'a str {
    match t {
        "average" => {
            if ["666", "777", "333bf", "444bf", "555bf", "333fm"].contains(&event) {
                "mean"
            } else {
                t
            }
        }
        _ => t,
    }
}

fn format_attempt_result(result: i32, event: &str) -> String {
    match event {
        "333fm" => {
            /* Note: FM singles are stored as the number of moves (e.g. 25),
            while averages are stored with 2 decimal places (e.g. 2533 for an average of 25.33 moves). */
            if result < 1000 {
                result.to_string()
            } else {
                centiseconds_to_time(result)
            }
        }
        "333mbf" => {
            if result >= 0 {
                let missed = result % 100;
                let points = 99 - ((result as f32 / 1e7).floor() as i32 % 100);
                let solved = points + missed;

                let attempted = solved + missed;

                let time = ((result as f32 / 100.).floor() % 1e5) as i32 * 100;

                format!(
                    "{solved}/{attempted} {}",
                    centiseconds_to_time(time).replace(".00", "")
                )
            } else {
                "0/0 0".to_string()
            }
        }
        _ => centiseconds_to_time(result),
    }
}

fn centiseconds_to_time(time: i32) -> String {
    let t = time as f32 / 100.;

    let min = (t / 60.).floor();

    let s = t - min * 60.;

    if min != 0. {
        format!("{min}:{s:04.2}")
    } else {
        format!("{s:.2}")
    }
}
