// thanks @pandaplusplus

use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use super::{async_read_poll, async_write_poll, AsyncRead, AsyncWrite, Delay};

//
//
//
pub trait AsyncReadWithTimeoutExt: AsyncRead {
    // ref https://github.com/rust-lang/futures-rs/blob/0.3.5/futures-util/src/io/mod.rs#L189
    fn read_with_timeout<'a>(
        &'a mut self,
        buf: &'a mut [u8],
        dur: Duration,
    ) -> ReadWithTimeout<'a, Self>
    where
        Self: Unpin,
    {
        ReadWithTimeout::new(self, buf, Delay::new(dur))
    }
}

// ref https://github.com/rust-lang/futures-rs/blob/0.3.5/futures-util/src/io/mod.rs#L382
impl<R: AsyncRead + ?Sized> AsyncReadWithTimeoutExt for R {}

// ref https://github.com/rust-lang/futures-rs/blob/0.3.5/futures-util/src/io/read.rs
#[derive(Debug)]
pub struct ReadWithTimeout<'a, R: ?Sized> {
    reader: &'a mut R,
    buf: &'a mut [u8],
    delay: Delay,
}

impl<R: ?Sized + Unpin> Unpin for ReadWithTimeout<'_, R> {}

impl<'a, R: AsyncRead + ?Sized + Unpin> ReadWithTimeout<'a, R> {
    fn new(reader: &'a mut R, buf: &'a mut [u8], delay: Delay) -> Self {
        Self { reader, buf, delay }
    }
}

impl<R: AsyncRead + ?Sized + Unpin> Future for ReadWithTimeout<'_, R> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;

        async_read_poll(&mut this.reader, &mut this.buf, &mut this.delay, cx)
    }
}

//
//
//

pub trait AsyncWriteWithTimeoutExt: AsyncWrite {
    // ref https://github.com/rust-lang/futures-rs/blob/0.3.5/futures-util/src/io/mod.rs#L425
    fn write_with_timeout<'a>(
        &'a mut self,
        buf: &'a [u8],
        dur: Duration,
    ) -> WriteWithTimeout<'a, Self>
    where
        Self: Unpin,
    {
        WriteWithTimeout::new(self, buf, Delay::new(dur))
    }
}

// ref https://github.com/rust-lang/futures-rs/blob/0.3.5/futures-util/src/io/mod.rs#L566
impl<W: AsyncWrite + ?Sized> AsyncWriteWithTimeoutExt for W {}

// ref https://github.com/rust-lang/futures-rs/blob/0.3.5/futures-util/src/io/write.rs
#[derive(Debug)]
pub struct WriteWithTimeout<'a, W: ?Sized> {
    writer: &'a mut W,
    buf: &'a [u8],
    delay: Delay,
}

impl<W: ?Sized + Unpin> Unpin for WriteWithTimeout<'_, W> {}

impl<'a, W: AsyncWrite + ?Sized + Unpin> WriteWithTimeout<'a, W> {
    fn new(writer: &'a mut W, buf: &'a [u8], delay: Delay) -> Self {
        Self { writer, buf, delay }
    }
}

impl<W: AsyncWrite + ?Sized + Unpin> Future for WriteWithTimeout<'_, W> {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;

        async_write_poll(&mut this.writer, &this.buf, &mut this.delay, cx)
    }
}
