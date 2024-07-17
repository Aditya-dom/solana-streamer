mod transactions {
    use geysers::{GeyserPlugin, Transaction};
    use log::info;
    use tokio_postgres::{Pool, NoTls as PGNoTls};

    async fn process_transactions(txns: Vec<Transaction>) -> Result<(), 
Box<dyn std::error::Error>> {
        for tx in txns {
            if let Some(err) = tx.error {
                // Handle error here
            }
        }
        Ok(())
    }
}
