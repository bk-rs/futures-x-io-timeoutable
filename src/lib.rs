#[cfg(any(feature = "futures_io", feature = "tokio02_io", feature = "tokio_io"))]
pub use futures_x_io;

#[cfg(feature = "futures_io")]
#[path = "futures_io.rs"]
pub mod futures_io;

#[cfg(feature = "tokio02_io")]
#[path = "tokio02_io.rs"]
pub mod tokio02_io;

#[cfg(feature = "tokio_io")]
#[path = "tokio_io.rs"]
pub mod tokio_io;

//
//
//
#[cfg(all(
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub use self::futures_io::rw;
#[cfg(all(
    feature = "futures_io",
    not(feature = "tokio02_io"),
    not(feature = "tokio_io")
))]
pub use self::futures_io::rw::{AsyncReadWithTimeoutExt, AsyncWriteWithTimeoutExt};

#[cfg(all(
    not(feature = "futures_io"),
    feature = "tokio02_io",
    not(feature = "tokio_io")
))]
pub use self::tokio02_io::rw;
#[cfg(all(
    not(feature = "futures_io"),
    feature = "tokio02_io",
    not(feature = "tokio_io")
))]
pub use self::tokio02_io::rw::{AsyncReadWithTimeoutExt, AsyncWriteWithTimeoutExt};

#[cfg(all(
    not(feature = "futures_io"),
    not(feature = "tokio02_io"),
    feature = "tokio_io"
))]
pub use self::tokio_io::rw;
#[cfg(all(
    not(feature = "futures_io"),
    not(feature = "tokio02_io"),
    feature = "tokio_io"
))]
pub use self::tokio_io::rw::{AsyncReadWithTimeoutExt, AsyncWriteWithTimeoutExt};
