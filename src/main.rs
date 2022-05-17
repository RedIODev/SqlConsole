use sql_console::{
    console::{evaluate_command, password_prompt, prompt, username_hostname_prompt},
    prelude::*,
    sql::SqlData,
};

fn main() -> ErrResult<()> {
    let args = get_validated_env_args()?;
    let (username, hostname) = match args {
        Some((first, second)) => (first, second),
        None => username_hostname_prompt()?,
    };
    let password = password_prompt(&username, &hostname)?;
    let mut sql_data = SqlData {
        username,
        hostname,
        password,
        database: None,
    };
    loop {
        let input = prompt(
            &sql_data.username,
            &sql_data.hostname,
            sql_data.database.as_deref(),
        )?;
        evaluate_command(&mut sql_data, &input);
    }
}
