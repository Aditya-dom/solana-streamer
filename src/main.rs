use lib::{DatabaseConn, transactions};
use geysers::{GeyserPlugin, Transaction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_conn = DatabaseConn::new().await?;
    let txns = vec![Transaction::new("tx1"), Transaction::new("tx2")];
    transactions::process_transactions(txns).await?;

    Ok(())
}