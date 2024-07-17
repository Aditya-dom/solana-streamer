mod database {
    use postgres::{NoTls, TlsConnection};

    let conn = NoTls::connect("host=... port=5432 dbname=mydatabase");

    fn execute_query(query: &str) -> Result<(), Box<dyn 
std::error::Error>> {
        conn.execute(query, &[])?;
        Ok(())
    }
}