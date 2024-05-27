use std::convert::Infallible;
use thiserror::Error;
use std::fmt;
use std::num::ParseIntError;
use tokio::sync::AcquireError;
use log::error;


#[derive(Debug, Error)]
pub enum Errors {
    #[error("Error: {0}")]
    Error(String),
    #[error("Join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("ToStr error: {0}")]
    ToStrError(#[from] hyper::header::ToStrError),
    #[error("Parse int error: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("AcquireError: {0}")]
    AcquireError(#[from] AcquireError),
    #[error("std::io::Error: {0}")]
    STDIOError(#[from] io::Error),
    #[error("Infallible: {0}")]
    Infallible(#[from] Infallible)
}
impl Errors {
    pub fn error(message: &str, err: impl fmt::Display) -> Self {
        error!("{} -> {}", message, err);
        Errors::Error(format!("{} -> {}", message, err))
    }
}
impl From<()> for Errors {
    fn from(_: ()) -> Self {
        Errors::Error("An unexpected error occurred".to_string())
    }
}



use std::io;
use std::str::FromStr;


pub fn input() -> Result<String, Errors> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).map_err(|e|{
        Errors::error("Ошибка ввода пользователя", e)
    })?;
    Ok(buffer.trim().to_string())
}
pub trait ParseInput<T> {
    fn parsing(&self) -> Result<T, Errors>;
}

impl<T> ParseInput<T> for String
    where
        T: FromStr,
        T::Err: std::fmt::Display,
{
    fn parsing(&self) -> Result<T, Errors> {
        self.parse::<T>().map_err(|e| {
            Errors::error("Ошибка парсинга", e)
        })
    }

}

impl<'a, T> ParseInput<T> for &'a str
    where
        T: FromStr,
        T::Err: std::fmt::Display,
{
    fn parsing(&self) -> Result<T, Errors> {
        self.parse::<T>().map_err(|e| {
            Errors::error("Ошибка парсинга", e)
        })
    }
}

