use geysers::{GeyserPlugin, Transaction};
use log::info;
use tokio_postgres::{Pool, NoTls as PGNoTls};

struct DatabaseConn {
    conn: PGNoTls,
}

impl DatabaseConn {
    fn new() -> Self {
        let mut conn = Pool::new("host=... port=5432 dbname=mydatabase 
user=Arawn password=mypass12345");
        DatabaseConn { conn }
    }

    async fn execute(&self, query: &str) -> Result<(), Box<dyn 
std::error::Error>> {
        self.conn.execute(query, &[]).await?;
        Ok(())
    }
}