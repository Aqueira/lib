mod custom_errors;

use std::io;
use std::str::FromStr;
use crate::custom_errors::Errors;

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
