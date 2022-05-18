use sql_console::{
    console::{evaluate_command, password_prompt, prompt, username_hostname_port_prompt},
    prelude::*,
    sql::SqlData,
};

fn main() -> ErrResult<()> {
    let args = get_validated_env_args()?;
    let (username, hostname, port) = match args {
        Some(args) => args,
        None => username_hostname_port_prompt()?,
    };
    let password = password_prompt(&username, &hostname, port)?;
    let mut sql_data = SqlData {
        username,
        hostname,
        port,
        password,
        database: None,
        connection: None,
    };
    loop {
        let input = prompt(&sql_data)?;
        evaluate_command(&mut sql_data, &input);
    }
}
