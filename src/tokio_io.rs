use futures_x_io::tokio_io::{AsyncRead, AsyncWrite};

//
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project_lite::pin_project;

pin_project! {
    #[derive(Debug)]
    struct Delay {
        #[pin]
        inner: tokio::time::Sleep,
    }
}
impl Delay {
    pub fn new(dur: std::time::Duration) -> Self {
        Self {
            inner: tokio::time::sleep(dur),
        }
    }
}
impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().inner.poll(cx)
    }
}

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
        Poll::Pending => {
            // ref https://github.com/tokio-rs/tokio/blob/tokio-1.1.0/tokio/src/macros/pin.rs#L127-L134
            #[allow(unused_mut)]
            let mut delay = delay;
            #[allow(unused_mut)]
            let mut delay = unsafe { Pin::new_unchecked(delay) };
            match delay.poll(cx) {
                Poll::Ready(_) => {
                    Poll::Ready(Err(io::Error::new(io::ErrorKind::TimedOut, "read timeout")))
                }
                Poll::Pending => Poll::Pending,
            }
        }
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
        Poll::Pending => {
            #[allow(unused_mut)]
            let mut delay = delay;
            #[allow(unused_mut)]
            let mut delay = unsafe { Pin::new_unchecked(delay) };
            match delay.poll(cx) {
                Poll::Ready(_) => Poll::Ready(Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "write timeout",
                ))),
                Poll::Pending => Poll::Pending,
            }
        }
    }
}

#[path = "rw.rs"]
pub mod rw;

//
use std::time::Duration;

pub async fn sleep(dur: Duration) {
    tokio::time::sleep(dur).await
}

#[path = "time.rs"]
pub mod time;
