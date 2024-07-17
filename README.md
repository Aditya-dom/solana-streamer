# solana-streamer

**Overview**

This is an advanced Solana transaction processor using Rust and 
async/await architecture to process transactions in parallel. It connects 
to the Solana node, listens for new transactions, deserializes them, and 
passes them on for further processing using multiple worker threads.
Generally, This library is for managing transactions in a PostgreSQL database 
using Tokio for asynchronous I/O. The library provides a simple interface 
for executing queries, tracking metrics, and handling errors.

**Components**

The library consists of several components:

* `DatabaseConn`: A struct that represents a connection to the PostgreSQL 
database.
* `transactions`: A module that contains functions for processing 
transactions.
* `metrics`: A module that contains functions for tracking metrics.
* `error`: A module that defines custom error types and implementations.

**Code Explanation**

### DatabaseConn

>This struct represents a connection to the PostgreSQL database. It has an `execute` method that takes a query string as input and executes it on the database using Tokio's async I/O capabilities.

```rust
struct DatabaseConn {
    conn: PGNoTls,
}

impl DatabaseConn {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
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
```

### transactions

>This module contains functions for processing transactions. The `process_transactions` function takes a vector of transactions as input and processes them asynchronously.

```rust
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
```

### metrics

>This module contains functions for tracking metrics. The `track_metric` function takes no input and simply sets a metric to a value.

```rust
mod metrics {
    use tokio_metrics::{Metric, NoTls as PGNoTls};

    let metric = Metric::new("transactions_processed", "int");

    async fn track_metric() -> Result<(), Box<dyn std::error::Error>> {
        metric.set(42);
        Ok(())
    }
}
```

### error

>This module defines custom error types and implementations. The `MyError` enum represents a custom error type that can be used to wrap other errors.

```rust
mod error {
    use tokio_error::{TokioError, TokioErrorKind};

    #[derive(Debug)]
    enum MyError {
        DatabaseError,
        ConfigError,
    }

    impl std::error::Error for MyError {}

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
{
            write!(f, "MyError")
        }
    }

    impl From<MyError> for TokioError {
        fn from(e: MyError) -> Self {
            TokioError::new(TokioErrorKind::Other, e)
        }
    }
}
```

**Geyser**
=====================

>A Rust library for managing transactions in a PostgreSQL database using Tokio for asynchronous I/O.

**Features**

* Asynchronous execution of queries
* Tracking of metrics and errors
* Custom error handling using the `error` module

**Usage**

1. Add the following dependencies to your `Cargo.toml` file:
```toml
[dependencies]
geysers = "0.2.0"
log = "0.4.14"
postgres = "0.19.7"
tokio = { version = "1", features = ["full"] }
snafu = { version = "0.7", features = ["backtraces"] }
tokio-postgres = { version = "0.7", features = ["with-serde_json-1"] }
env_logger = "0.9"
futures = "0.3"
serde = "1.0"
serde_json = "1.0"
```
2. Import the necessary modules in your code:
```rust
use lib::{DatabaseConn, transactions};
use geysers::{GeyserPlugin, Transaction};
```
3. Create a `DatabaseConn` instance and use it to execute queries:
```rust
let db_conn = DatabaseConn::new().await?;
db_conn.execute("SELECT * FROM mytable").await?;
```
4. Use the `transactions` module to process transactions:
```rust
let txns = vec![Transaction::new("tx1"), Transaction::new("tx2")];
transactions::process_transactions(txns).await?;
```