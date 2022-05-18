use std::{env, error::Error, fmt::Display};

use crate::set_foreground_color;

pub const COLOR_USER: &str = set_foreground_color!(22, 198, 12);
pub const COLOR_DB: &str = set_foreground_color!(59, 120, 255);
pub const COLOR_NORMAL: &str = set_foreground_color!(204, 204, 204);
pub const COLOR_CONNECTED: &str = set_foreground_color!(0, 255, 255);
pub const COLOR_DISCONNECTED: &str = set_foreground_color!(206,10,24);

pub fn get_validated_env_args() -> ErrResult<Option<(String, String,u16)>> {
    let args = env::args();

    let mut args = args.skip(1);

    let first = args.next();

    if first == None {
        return Ok(None);
    }

    let second = args.next();

    let error: Box<GenericError> = Box::<GenericError>::new(
        String::from("Invalid number of arguments. Expected 0 or 3.").into(),
    );

    if second == None {
        return Err(error);
    }

    let third = args.next();

    if third == None {
        
    }

    let third = third.unwrap().parse::<u16>()?; 

    let fourth = args.next();

    if fourth == None {
        return Ok(Some((first.unwrap(), second.unwrap(),third)));
    }
    Err(error)
}

pub type ErrResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct GenericError(String);

impl Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrMsg:{}", self.0)
    }
}

impl Error for GenericError {}

impl From<String> for GenericError {
    fn from(msg: String) -> Self {
        Self(msg)
    }
}
