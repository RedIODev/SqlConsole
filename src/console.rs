use std::io::Write;

use rpassword::read_password;

use crate::{
    ansi::{cursor, erase},
    sql::SqlData,
};

use super::prelude::*;

pub fn prompt(sql_data:&SqlData) -> ErrResult<String> {
    print!(
        "{}MYSQL {}{}@{}:{}{}:{}{}{}$ ",
        if sql_data.connection.is_some() {COLOR_CONNECTED} else {COLOR_DISCONNECTED},
        COLOR_USER,
        sql_data.username,
        sql_data.hostname,
        sql_data.port,
        COLOR_NORMAL,
        COLOR_DB,
        sql_data.database.as_deref().unwrap_or("~"),
        COLOR_NORMAL
    );
    std::io::stdout().flush()?;
    input()
}

pub fn password_prompt(username: &str, hostname: &str, port:u16) -> ErrResult<String> {
    print!("{}{}@{}:{}'s password: ", COLOR_NORMAL, username, hostname, port);
    std::io::stdout().flush()?;
    Ok(read_password()?)
}

pub fn username_hostname_port_prompt() -> ErrResult<(String, String, u16)> {
    print!("Username: ");
    std::io::stdout().flush()?;
    let username = input()?;
    print!("Hostname: ");
    std::io::stdout().flush()?;
    let hostname = input()?;
    let port = loop {
        print!("Port: ");
        std::io::stdout().flush()?;
        let input = input()?;
        if let Ok(port) = input.parse::<u16>() {
            break port;
        }
        println!("Invalid port \"{}\". Only 0-65535 are valid.", input);
    };
    Ok((username, hostname, port))
}

pub fn evaluate_command(sql_data: &mut SqlData, input: &str) {
    let input = input.trim_end();
    match input {
        "exit" => {
            println!("Bye");
            std::process::exit(0);
        }

        s if s.starts_with("cd") => {
            let arg = s.strip_prefix("cd").unwrap().trim_start().trim_end();
            sql_data.database = if arg.is_empty() {
                None
            } else {
                Some(arg.to_owned())
            };
        }

        s if s.starts_with("cls") => {
            print!("{}{}", cursor::HOME, erase::ALL_AFTER_CUR);
        }

        s if s.trim_start().trim_end().is_empty() => {}

        s if s.starts_with("con") => {
            sql_data.connect();
        }

        s if s.starts_with("#") => {

        }
        s => {
            println!(
                "Invalid command: \"{}\"",
                s.trim_start().split_terminator(" ").next().unwrap()
            )
        }
    };
}

fn input() -> ErrResult<String> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_owned())
}
