use chrono::Local;
use futures::StreamExt;
use miniserde::{Deserialize, Serialize, json};
use reqwest::Client;
use std::{collections::HashSet, fs, io, path::PathBuf, sync::LazyLock};
use thiserror::Error;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
struct Competition {
    id: String,
    name: String,
    registration_open: String,
    registration_close: String,
    start_date: String,
    end_date: String,
    competitor_limit: u16,
    cancelled_at: Option<String>,
    url: String,
    city: String,
    venue_address: String,
    latitude_degrees: String,
    longitude_degrees: String,
    country_iso2: String,
    events_ids: Vec<String>,
}

pub struct Competitions {
    data: HashSet<Competition>,
}

#[derive(Error, Debug)]
enum InitError {
    #[error("Io Error")]
    Io(#[from] io::Error),
    #[error("Parse error")]
    MiniSerde(#[from] miniserde::Error),
    #[error("Request error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("ToStr error")]
    ToStr(#[from] reqwest::header::ToStrError),
    #[error("Missing header")]
    MissingHeader,
}

type Result<T> = std::result::Result<T, InitError>;

pub static COMPS: LazyLock<Competitions> = LazyLock::new(|| Competitions::init().unwrap());

impl Competitions {
    const PATH: &str = "./data/wcacomps.json";
    const URL: &str =
        "https://www.worldcubeassociation.org/api/v0/competitions?sort=start_date&start=";

    pub fn init() -> Result<Self> {
        let Ok(v) = Self::read() else {
            return Self::download().await;
        };

        Ok(v)
    }

    fn read() -> Result<Self> {
        let path = PathBuf::from(Self::PATH);

        let data = fs::read_to_string(path)?;

        Ok(Self {
            data: HashSet::from_iter(json::from_str::<Vec<Competition>>(&data)?.into_iter()),
        })
    }

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

        Ok(Self {
            data: data
                .into_iter()
                .flat_map(|d| json::from_str::<Vec<Competition>>(&d).unwrap())
                .collect(),
        })
    }

    async fn update(&mut self) -> Result<Self> {
        let path = PathBuf::from(Self::PATH);

        let new = Self::download().await?;

        let diff = self.data.symmetric_difference(&new.data).collect();

        *self = new;

        let data = json::to_string(&Vec::from_iter(self.data.into_iter()));

        fs::write(path, data)?;

        Ok(Self { data: diff })
    }
}

/*
const getUpcomingCompetitions = async () => {
  const url = `https://www.worldcubeassociation.org/api/v0/competitions?sort=start_date&start=${dayjs().format(
    "YYYY-MM-DD",
  )}`;

  const { response: firstResponse, numberOfPages } = await fetch(url).then(
    async (res) => {
      const numberOfPages = Math.ceil(
        res.headers.get("Total") / res.headers.get("Per-page"),
      );
      const response = await res.json();
      return { response, numberOfPages };
    },
  );

  const responseArray = await Promise.all(
    map(
      (n) => fetch(url + "&page=" + n).then((res) => res.json()),
      range(2, numberOfPages + 1),
    ),
  );

  return map(
    pick([
      "id",
      "name",
      "registration_open",
      "registration_close",
      "start_date",
      "end_date",
      "competitor_limit",
      "cancelled_at",
      "url",
      "city",
      "venue_address",
      "latitude_degrees",
      "longitude_degrees",
      "country_iso2",
      "event_ids",
    ]),
    flatten([firstResponse, responseArray]),
  );
};
*/
