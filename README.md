# futures-rustls
[![crates](https://img.shields.io/crates/v/futures-rustls.svg)](https://crates.io/crates/futures-rustls)
[![docs.rs](https://docs.rs/futures-rustls/badge.svg)](https://docs.rs/futures-rustls/)

Asynchronous TLS/SSL streams for futures using
[Rustls](https://github.com/rustls/rustls).

### Basic Structure of a Client

```rust
use webpki_roots::TLS_SERVER_ROOTS;
use futures_rustls::{
    TlsConnector,
    rustls::{ClientConfig, RootCertStore, pki_types::ServerName},
};

// ...

let mut root_store = RootCertStore::empty();
root_store.extend(TLS_SERVER_ROOTS.iter().cloned());

let config = ClientConfig::builder()
    .with_root_certificates(root_store)
    .with_no_client_auth();

let config = TlsConnector::from(Arc::new(config));
let dnsname = ServerName::try_from("www.rust-lang.org").expect("Invalid DNS name.");

let stream = TcpStream::connect(&addr).await?;
let mut stream = config.connect(dnsname, stream).await?;


// ...
```

### License & Origin

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

This started as a fork of [tokio-rustls](https://github.com/rustls/tokio-rustls).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in futures-rustls by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
