use futures_rustls::{
    client::TlsStream,
    pki_types::ServerName,
    rustls::{self, ClientConfig},
    TlsConnector,
};
use futures_util::io::{AsyncReadExt, AsyncWriteExt};
use smol::net::TcpStream;
use std::convert::TryFrom;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Arc;

async fn get(
    config: Arc<ClientConfig>,
    domain: &str,
    port: u16,
) -> io::Result<(TlsStream<TcpStream>, String)> {
    let connector = TlsConnector::from(config);
    let input = format!("GET / HTTP/1.0\r\nHost: {}\r\n\r\n", domain);

    let addr = (domain, port).to_socket_addrs()?.next().unwrap();
    let domain = ServerName::try_from(domain).unwrap().to_owned();
    let mut buf = Vec::new();

    let stream = TcpStream::connect(&addr).await?;
    let mut stream = connector.connect(domain, stream).await?;
    stream.write_all(input.as_bytes()).await?;
    stream.flush().await?;
    stream.read_to_end(&mut buf).await?;

    Ok((stream, String::from_utf8(buf).unwrap()))
}

#[test]
fn test_tls12() -> io::Result<()> {
    let fut = async {
        let root_store = rustls::RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
        };
        let config =
            rustls::ClientConfig::builder_with_protocol_versions(&[&rustls::version::TLS12])
                .with_root_certificates(root_store)
                .with_no_client_auth();

        let config = Arc::new(config);
        let domain = "tls-v1-2.badssl.com";

        let (_, output) = get(config.clone(), domain, 1012).await?;
        assert!(
            output.contains("<title>tls-v1-2.badssl.com</title>"),
            "failed badssl test, output: {}",
            output
        );

        Ok(())
    };

    smol::block_on(fut)
}

#[ignore]
#[should_panic]
#[test]
fn test_tls13() {
    unimplemented!("todo https://github.com/chromium/badssl.com/pull/373");
}

#[test]
fn test_modern() -> io::Result<()> {
    let fut = async {
        let root_store = rustls::RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
        };
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        let config = Arc::new(config);
        let domain = "mozilla-modern.badssl.com";

        let (_, output) = get(config.clone(), domain, 443).await?;
        assert!(
            output.contains("<title>mozilla-modern.badssl.com</title>"),
            "failed badssl test, output: {}",
            output
        );

        Ok(())
    };

    smol::block_on(fut)
}
