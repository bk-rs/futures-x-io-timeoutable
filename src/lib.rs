#[cfg(all(
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub mod rw;
#[cfg(all(
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub use rw::{AsyncReadWithTimeoutExt, AsyncWriteWithTimeoutExt};

#[cfg(all(
    not(feature = "futures_io"),
    feature = "tokio02_io",
    not(feature = "tokio_io")
))]
pub mod rw;
#[cfg(all(
    not(feature = "futures_io"),
    feature = "tokio02_io",
    not(feature = "tokio_io")
))]
pub use rw::{AsyncReadWithTimeoutExt, AsyncWriteWithTimeoutExt};

#[cfg(all(
    not(feature = "futures_io"),
    not(feature = "tokio02_io"),
    feature = "tokio_io"
))]
pub mod rw;
#[cfg(all(
    not(feature = "futures_io"),
    not(feature = "tokio02_io"),
    feature = "tokio_io"
))]
pub use rw::{AsyncReadWithTimeoutExt, AsyncWriteWithTimeoutExt};
