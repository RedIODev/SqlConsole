use mysql::{PooledConn, Pool,Opts, OptsBuilder, prelude::Queryable};

use crate::prelude::ErrResult;

pub struct SqlData {
    pub username:String,
    pub hostname:String,
    pub port:u16,
    pub password:String,
    pub database:Option<String>,
}

impl SqlData {
    pub fn connect(&self) -> ErrResult<PooledConn> {
        // let database = match &self.database {
        //     Some(database) => database,
        //     None => return Err(Box::<GenericError>::new("No database selected.".to_owned().into())),
        // };

        let opts: Opts = OptsBuilder::new()
                .user(Some(&self.username))
                .pass(Some(&self.password))
                .tcp_port(self.port)
                .ip_or_hostname(Some(&self.hostname))
                .db_name(self.database.as_ref())
                .into();
        Ok(Pool::new(opts)?.get_conn()?)
    }

    pub fn evaluate_sql(&self,command: &str) -> ErrResult<()> {
        let mut con = self.connect()?;
        let mut result = con.query_iter(command)?;
        while let Some(result_set) = result.iter() {
            for row in result_set{
                println!("{:?}",(row?))
            }
        }
        Ok(())
    }
}