#[cfg(all(
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
mod time_futures_io_tests {
    use std::error;
    use std::time::{Duration, Instant};

    use futures_lite::future::block_on;

    use futures_x_io_timeoutable::sleep;

    #[test]
    fn test_sleep() -> Result<(), Box<dyn error::Error>> {
        block_on(async {
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
