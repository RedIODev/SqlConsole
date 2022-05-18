use mysql::PooledConn;

pub struct SqlData {
    pub username:String,
    pub hostname:String,
    pub port:u16,
    pub password:String,
    pub database:Option<String>,
    pub connection:Option<PooledConn>,
}

impl SqlData {
    pub fn connect(&mut self) {
        
    }
}