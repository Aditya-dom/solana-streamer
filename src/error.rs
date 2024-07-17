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
