
use std::{error::Error, fmt::Display};

use self::constants::{COLOR_DB, COLOR_NORMAL, COLOR_USER};

pub mod ansi;
pub mod constants;

pub fn prompt(username: &str, hostname: &str, database: &str) -> String {
    format!(
        "{}{}@{}{}:{}{}{}$ ",
        COLOR_USER, username, hostname, COLOR_NORMAL, COLOR_DB, database, COLOR_NORMAL
    )
}

pub fn password_prompt(username: &str, hostname: &str) -> String {
    format!("{}{}@{}'s password: ", COLOR_NORMAL, username, hostname)
}

pub type TError = Box<dyn Error>;
#[derive(Debug)]
pub struct EnvError(String);

impl EnvError {
    pub fn from(err_string: String) -> Box<EnvError> {
        Box::new(EnvError(err_string))
    }
}

impl Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0)
    }
}

impl Error for EnvError {}

pub fn parse_env_args(mut args: Vec<String>) -> Result<(String, String),TError> {
    let len = args.len();
    if len != 3 { return Err(EnvError::from(format!("Invalid argument length: {}, only 3 are permitted",len)))}
    Ok((args.remove(1),args.remove(1)))
}