#[cfg(all(
    not(feature = "futures_io"),
    not(feature = "tokio02_io"),
    feature = "tokio_io"
))]
mod rw_tokio02_io_tests {
    use std::io;
    use std::time::{Duration, Instant};

    use tokio::net::{TcpListener, TcpStream};
    use tokio::runtime::Runtime;

    use futures_x_io_timeoutable::{AsyncReadWithTimeoutExt, AsyncWriteWithTimeoutExt};

    #[test]
    fn simple() -> io::Result<()> {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let listener = TcpListener::bind("127.0.0.1:0").await?;
            let addr = listener.local_addr()?;

            let mut tcp_stream_c = TcpStream::connect(addr).await?;
            let (mut tcp_stream_s, _) = listener.accept().await.expect("Accept failed");

            tcp_stream_s
                .write_with_timeout(b"foo", Duration::from_secs(1))
                .await?;

            let mut buf = vec![0u8; 5];
            let n = tcp_stream_c
                .read_with_timeout(&mut buf, Duration::from_secs(1))
                .await?;
            assert_eq!(n, 3);
            assert_eq!(buf, b"foo\0\0");

            let instant = Instant::now();
            let two_secs = Duration::from_secs(2);
            let three_secs = Duration::from_secs(3);
            let err = tcp_stream_c
                .read_with_timeout(&mut buf, Duration::from_secs(2))
                .await
                .err()
                .unwrap();
            assert!(instant.elapsed() >= two_secs);
            assert!(instant.elapsed() < three_secs);
            assert_eq!(err.kind(), io::ErrorKind::TimedOut);
            assert_eq!(err.to_string(), "read timeout");

            Ok(())
        })
    }
}
