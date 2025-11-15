use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, InitError>;

#[derive(Error, Debug)]
pub enum InitError {
    #[error("Io Error")]
    Io(#[from] io::Error),
    #[error("Parse error")]
    Serde(#[from] serde_json::Error),
    #[error("Request error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("ToStr error")]
    ToStr(#[from] reqwest::header::ToStrError),
    #[error("Missing header")]
    MissingHeader,
    #[error("Missing data")]
    MissingData,
    #[error("Cynic req error")]
    CynicReq(#[from] cynic::http::CynicReqwestError),
    #[error("Missing channel")]
    MissingChannel,
    #[error("Serenity error")]
    Serenity,
    #[error("Env error")]
    Env(#[from] std::env::VarError),
}
