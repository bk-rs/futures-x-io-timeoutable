#[cfg(any(feature = "futures_io", feature = "tokio02_io", feature = "tokio_io"))]
pub use futures_x_io;

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
pub use rw as futures_io_rw;
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
pub use rw as tokio02_io_rw;
#[cfg(all(
    not(feature = "futures_io"),
    feature = "tokio02_io",
    not(feature = "tokio_io")
))]
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
pub use rw as tokio_io_rw;
#[cfg(all(
    not(feature = "futures_io"),
    not(feature = "tokio02_io"),
    feature = "tokio_io"
))]
pub use rw::{AsyncReadWithTimeoutExt, AsyncWriteWithTimeoutExt};
