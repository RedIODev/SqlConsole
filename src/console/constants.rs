use const_format::concatcp;

use crate::set_foreground_color;

const COLOR_USER:&str = set_foreground_color!(22,198,12);
const COLOR_DB:&str = set_foreground_color!(59,120,255);
const COLOR_NORMAL:&str = set_foreground_color!(204,204,204);

pub fn prompt(username: &str,hostname:&str,database:&str) -> String {
    format!("{}{}@{}{}:{}{}{}$ ",COLOR_USER,username,hostname,COLOR_NORMAL,COLOR_DB,database,COLOR_NORMAL)
}

pub const PROMPT: &str = concatcp!(COLOR_USER, "{}@{}", COLOR_NORMAL, ":", COLOR_DB ,"{}", COLOR_NORMAL, "$ "); //username, hostname, database
pub const PASS_PROMPT: &str = concatcp!(COLOR_NORMAL, "{}@{}'s password: "); //username, hostname