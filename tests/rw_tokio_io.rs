#[cfg(all(
    not(feature = "futures_io"),
    any(feature = "tokio02_io", feature = "tokio_io")
))]
mod rw_tokio_io_tests {
    use std::io;
    use std::time::{Duration, Instant};

    #[cfg(all(not(feature = "tokio02_io"), feature = "tokio_io"))]
    use tokio::{
        net::{TcpListener, TcpStream},
        runtime::Runtime,
    };
    #[cfg(all(feature = "tokio02_io", not(feature = "tokio_io")))]
    use tokio02::{
        net::{TcpListener, TcpStream},
        runtime::Runtime,
    };

    use futures_x_io_timeoutable::rw::AsyncWriteWithTimeoutExt;
    use futures_x_io_timeoutable::AsyncReadWithTimeoutExt;

    #[test]
    fn simple() -> io::Result<()> {
        #[cfg(all(not(feature = "tokio02_io"), feature = "tokio_io"))]
        let rt = Runtime::new().unwrap();
        #[cfg(all(feature = "tokio02_io", not(feature = "tokio_io")))]
        let mut rt = Runtime::new().unwrap();

        rt.block_on(async {
            #[cfg(all(not(feature = "tokio02_io"), feature = "tokio_io"))]
            let listener = TcpListener::bind("127.0.0.1:0").await?;
            #[cfg(all(feature = "tokio02_io", not(feature = "tokio_io")))]
            let mut listener = TcpListener::bind("127.0.0.1:0").await?;

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
