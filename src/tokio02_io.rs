use futures_x_io::tokio02_io::{AsyncRead, AsyncWrite};

//
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug)]
struct Delay {
    inner: tokio02::time::Delay,
}
impl Delay {
    pub fn new(dur: std::time::Duration) -> Self {
        Self {
            inner: tokio02::time::delay_for(dur),
        }
    }
}
impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        Pin::new(&mut this.inner).poll(cx)
    }
}

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

fn async_write_poll<W: AsyncWrite + ?Sized + Unpin>(
    writer: &mut W,
    buf: &[u8],
    delay: &mut Delay,
    cx: &mut Context<'_>,
) -> Poll<io::Result<usize>> {
    let poll_ret = Pin::new(writer).poll_write(cx, buf);

    match poll_ret {
        Poll::Ready(ret) => Poll::Ready(ret),
        Poll::Pending => match Pin::new(delay).poll(cx) {
            Poll::Ready(_) => Poll::Ready(Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "write timeout",
            ))),
            Poll::Pending => Poll::Pending,
        },
    }
}

#[path = "rw.rs"]
pub mod rw;
