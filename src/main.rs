
use rpassword::read_password;
use rusqlite::Connection;

fn main() -> Result<(),TError> {
    let (username, hostname) = parse_env_args(std::env::args().collect())?;
    print!("{}", password_prompt(&username, &hostname));
    let password = read_password()?;
    loop {
        println!("{}",prompt(&username,&hostname,"~"));
        
    }
    Ok(())
}