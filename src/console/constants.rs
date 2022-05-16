use const_format::concatcp;

use crate::set_foreground_color;

pub const PROMPT: &str = concatcp!(set_foreground_color!(0,0,0), "{}@{}", ":{}$ "); //username, hostname, database
pub const PASS_PROMPT: &str = "{}@{}'s password: "; //username, hostname