use super::TError;


pub struct SqlData {
    pub username:String,
    pub hostname:String,
    pub password:String,
    pub database:Option<String>,
}

impl SqlData {
    pub fn evaluate_command(&mut self, input:&str) -> Result<(),TError> {
        let input = input.trim_end();
        match input {
            "exit" => {
                println!("Bye");
                std::process::exit(0);
            },

            s if s.starts_with("cdb") => {
                self.database = Some(s.strip_prefix("cdb").unwrap().trim_start().to_owned())
            }
            _ => {

            }
        }
        Ok(())
    }
}