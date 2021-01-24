#[cfg(all(
    not(feature = "futures_io"),
    any(feature = "tokio02_io", feature = "tokio_io")
))]
mod time_tokio_io_tests {
    use std::error;
    use std::time::{Duration, Instant};

    #[cfg(all(not(feature = "tokio02_io"), feature = "tokio_io"))]
    use tokio::runtime::Runtime;
    #[cfg(all(feature = "tokio02_io", not(feature = "tokio_io")))]
    use tokio02::runtime::Runtime;

    use futures_x_io_timeoutable::sleep;

    #[test]
    fn test_sleep() -> Result<(), Box<dyn error::Error>> {
        #[cfg(all(not(feature = "tokio02_io"), feature = "tokio_io"))]
        let rt = Runtime::new().unwrap();
        #[cfg(all(feature = "tokio02_io", not(feature = "tokio_io")))]
        let mut rt = Runtime::new().unwrap();

        rt.block_on(async {
            let instant = Instant::now();
            let two_secs = Duration::from_secs(2);
            let three_secs = Duration::from_secs(3);
            sleep(Duration::from_secs(2)).await;
            assert!(instant.elapsed() >= two_secs);
            assert!(instant.elapsed() < three_secs);

            Ok(())
        })
    }
}
