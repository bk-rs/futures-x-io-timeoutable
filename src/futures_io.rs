use futures_x_io::futures_io::{AsyncRead, AsyncWrite};

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
    let poll_ret = Pin::new(reader).poll_read(cx, buf);

    match poll_ret {
        Poll::Ready(ret) => Poll::Ready(ret),
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
