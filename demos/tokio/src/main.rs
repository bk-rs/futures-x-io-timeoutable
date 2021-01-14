/*
cargo run -p futures-x-io-timeoutable-demo-tokio
*/

use std::io;
use std::io::Cursor;
use std::time::Duration;

use futures_x_io_timeoutable::tokio_io_rw::AsyncReadWithTimeoutExt;

#[tokio::main]
async fn main() -> io::Result<()> {
    run().await
}

async fn run() -> io::Result<()> {
    //
    let mut cursor = Cursor::new(b"foo".to_vec());

    let mut buf = vec![0u8; 5];
    let n = cursor
        .read_with_timeout(&mut buf, Duration::from_secs(1))
        .await?;
    assert_eq!(n, 3);
    assert_eq!(buf, b"foo\0\0");

    println!("done");

    Ok(())
}
