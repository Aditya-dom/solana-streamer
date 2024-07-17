mod metrics {
    use tokio_metrics::{Metric, NoTls as PGNoTls};

    let metric = Metric::new("transactions_processed", "int");

    async fn track_metric() -> Result<(), Box<dyn std::error::Error>> {
        metric.set(42);
        Ok(())
    }
}