use std::io::Write;

use rpassword::read_password;

use crate::{sql::SqlData, ansi::{cursor, erase}};

use super::prelude::*;

pub fn prompt(username: &str, hostname: &str, database: Option<&str>) -> ErrResult<String> {
    print!(
        "{}DB {}{}@{}{}:{}{}{}$ ",
        COLOR_PREFIX ,COLOR_USER, username, hostname, COLOR_NORMAL, COLOR_DB, database.unwrap_or("~"), COLOR_NORMAL
    );
    std::io::stdout().flush()?;
    input()
}

pub fn password_prompt(username: &str, hostname: &str) -> ErrResult<String> {
    print!("{}{}@{}'s password: ", COLOR_NORMAL, username, hostname);
    std::io::stdout().flush()?;
    Ok(read_password()?)
}

pub fn username_hostname_prompt() -> ErrResult<(String, String)> {
    print!("Username: ");
    std::io::stdout().flush()?;
    let username = input()?;
    print!("Hostname: ");
    std::io::stdout().flush()?;
    let hostname = input()?;
    Ok((username, hostname))
}

pub fn evaluate_command(sql_data:&mut SqlData, input:&str) -> ErrResult<()> {
    let input = input.trim_end();
    match input {
        "exit" => {
            println!("Bye");
            std::process::exit(0);
        },

        s if s.starts_with("cd") => {
            let arg = s.strip_prefix("cd").unwrap().trim_start().trim_end();
            sql_data.database = if arg.is_empty() {None} else {Some(arg.to_owned())};
        }

        s if s.starts_with("cls") => {
            print!("{}{}",cursor::HOME, erase::ALL_AFTER_CUR);
        }

        s if s.trim_start().trim_end().is_empty() => {}

        s => {
            println!("Invalid command: \"{}\"",s.trim_start().split_terminator(" ").next().unwrap())
        }
    }
    Ok(())
}

fn input() -> ErrResult<String> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_owned())
}