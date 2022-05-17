
use std::io::Write;

use rpassword::read_password;
use sql_console::console::{TError, parse_env_args, password_prompt, prompt, sql::SqlData};

fn main() -> Result<(),TError> {
    let (username, hostname) = parse_env_args(std::env::args().collect())?;
    print!("{}", password_prompt(&username, &hostname));
    std::io::stdout().flush()?;
    let password = read_password()?;
    let mut sql_data = SqlData {
        username,
        hostname,
        password,
        database:None
    };
    loop {
        print!("{}",prompt(&sql_data.username,&sql_data.hostname,&sql_data.database.as_ref().unwrap_or(&"~".to_owned())));
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        sql_data.evaluate_command(&input)?;
    }
}