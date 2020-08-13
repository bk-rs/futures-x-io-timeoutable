cfg_if::cfg_if! {
    if #[cfg(all(feature = "futures_io", not(feature = "tokio_io")))] {
        pub mod rw;
        pub use rw::{AsyncReadWithTimeoutExt, AsyncWriteWithTimeoutExt};
    } else if #[cfg(all(not(feature = "futures_io"), feature = "tokio_io"))] {
        pub mod rw;
        pub use rw::{AsyncReadWithTimeoutExt, AsyncWriteWithTimeoutExt};
    }
}
