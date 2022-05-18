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
        COLOR_LOGO,
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
    let port = input_port()?;
    Ok((username, hostname, port))
}

pub fn evaluate_command(sql_data: &mut SqlData, input: &str) -> ErrResult<()> {
    let input = input.trim_end();
    match input {
        "exit" => {
            println!("Bye");
            std::process::exit(0);
        }

        s if s.starts_with("cd") => {
            let arg = trim_arg(s,"cd");
            sql_data.database = if arg.is_empty() {
                None
            } else {
                Some(arg.to_owned())
            };
        }

        s if s.starts_with("cu") => {
            let arg = trim_arg(s, "cu");
            sql_data.username = arg.to_owned();
            sql_data.password = password_prompt(&sql_data.username,&sql_data.hostname,sql_data.port)?;
        }

        s if s.starts_with("ch") => {
            let arg = trim_arg(s, "ch");
            sql_data.hostname = arg.to_owned();
        }

        s if s.starts_with("cp") => {
            sql_data.port = input_port()?;
        }

        s if s.starts_with("cls") => {
            print!("{}{}", cursor::HOME, erase::ALL_AFTER_CUR);
        }

        s if s.trim_start().trim_end().is_empty() => {}

        s => {
            sql_data.evaluate_sql(s)?;
        }
    };
    Ok(())
}

fn input() -> ErrResult<String> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_owned())
}

fn trim_arg<'a>(input: &'a str, command: &str) -> &'a str {
    input.strip_prefix(command).unwrap().trim_start().trim_end()
}

fn input_port() -> ErrResult<u16> {
    Ok(loop {
        print!("Port: ");
        std::io::stdout().flush()?;
        let input = input()?;
        if let Ok(port) = input.parse::<u16>() {
            break port;
        }
        println!("Invalid port \"{}\". Only 0-65535 are valid.", input);
    })
}