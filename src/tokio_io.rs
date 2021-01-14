use futures_x_io::tokio_io::{AsyncRead, AsyncWrite};

//
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_timer::Delay;

fn async_read_poll<R: AsyncRead + ?Sized + Unpin>(
    reader: &mut R,
    buf: &mut [u8],
    delay: &mut Delay,
    cx: &mut Context<'_>,
) -> Poll<io::Result<usize>> {
    // ref https://github.com/tokio-rs/tokio/blob/tokio-1.0.1/tokio/src/io/util/read.rs#L51-L53

    let mut buf = futures_x_io::tokio_io::ext::ReadBuf::new(buf);
    let poll_ret = Pin::new(reader).poll_read(cx, &mut buf);

    match poll_ret {
        Poll::Ready(Ok(_)) => Poll::Ready(Ok(buf.filled().len())),
        Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
        Poll::Pending => match Pin::new(delay).poll(cx) {
            Poll::Ready(_) => {
                Poll::Ready(Err(io::Error::new(io::ErrorKind::TimedOut, "read timeout")))
            }
            Poll::Pending => Poll::Pending,
        },
    }
}

#[path = "rw.rs"]
pub mod rw;
