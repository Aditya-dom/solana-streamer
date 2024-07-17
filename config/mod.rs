mod config {
    use tokio_config::{Config, NoTls as PGNoTls};

    let config = Config::new("path/to/config/file");

    async fn get_config() -> Result<serde_json::Value, Box<dyn 
std::error::Error>> {
        config.get("some_key")?;
        Ok(())
    }
}