# futures-x-io-timeoutable

* [Cargo package](https://crates.io/crates/futures-x-io-timeoutable)

## Examples

* [async-std](demos/async-std/src/main.rs)
* [tokio](demos/tokio/src/main.rs)

## Dev

```
cargo clippy --all --all-features -- -D clippy::all
cargo +nightly clippy --all --all-features -- -D clippy::all

cargo fmt --all -- --check
```

```
cargo build-all-features

╰─➤ cargo test-all-features -- --nocapture 2>/dev/null | grep 'test rw_' | grep ' ... ok' | wc -l
3
```

```
cargo run -p futures-x-io-timeoutable-demo-async-std

cargo run -p futures-x-io-timeoutable-demo-tokio
```
