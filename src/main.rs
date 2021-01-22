use clap::{Arg, App, ArgMatches, crate_authors};
use std::path::Path;
use tonic::transport::{Server, Identity, ServerTlsConfig};
use tokio::net::UnixListener;

use futures::TryFutureExt;

mod runtime_service;
use runtime_service::new_runtime_service_server;
mod image_service;
use image_service::new_image_service_server;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let matches = App::new("rust-cri")
        .version("0.1")
        .author(crate_authors!())
        .arg(
            Arg::new("tls")
                .about("Enable tls")
                .short('t')
                .long("tls"))
        .arg(
            Arg::new("tls-cert")
                .about("Specify the TLS certificate to use")
                .long("tls-cert")
                .default_value("data/server.pem"))
        .arg(
            Arg::new("tls-key")
                .about("Specity the TLS key to use")
                .long("tls-key")
                .default_value("data/server.key"))
        .arg(
            Arg::new("url")
                .about("The url endpoint to open, can be unix:// or tcp://")
                .index(1)
                .default_value("tcp://0.0.0.0:50051")
                .validator(is_valid_url))
        .get_matches();

    let url = matches.value_of("url").unwrap();

    if url.starts_with("unix://") {
        unix_socket_server(&matches).await?;
    } else if url.starts_with("tcp://") {
        tcp_server(&matches).await?;
    }

    Ok(())
}

fn is_valid_url(url: &str) -> Result<(), String> {
    if url.starts_with("unix://") || url.starts_with("tcp://") {
        Ok(())
    } else {
        Err(String::from("only unix:// or tcp:// prefix supported"))
    }
}

async fn tcp_server(args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> { 
    let addr = args.value_of("url").unwrap().strip_prefix("tcp://").unwrap();
    Server::builder()
        .add_service(new_runtime_service_server())
        .add_service(new_image_service_server())
        .serve(addr.parse()?)
        .await?;

    Ok(())
}

#[cfg(unix)]
async fn unix_socket_server(args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let path = args.value_of("url").unwrap().strip_prefix("tcp://").unwrap();
    let cert = tokio::fs::read(args.value_of("tls-cert").unwrap()).await?;
    let key = tokio::fs::read(args.value_of("tls-key").unwrap()).await?;

    let identity = Identity::from_pem(cert, key);

    tokio::fs::create_dir_all(Path::new(path).parent().unwrap()).await?;

    let incoming = {
        let uds = UnixListener::bind(path)?;

        async_stream::stream! {
            while let item = uds.accept().map_ok(|(st, _)| unix::UnixStream(st)).await {
                yield item;
            }
        }
    };

    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(new_runtime_service_server())
        .add_service(new_image_service_server())
        .serve_with_incoming(incoming)
        .await?;

    Ok(())
}

#[cfg(unix)]
mod unix {
    use std::{
        pin::Pin,
        task::{Context, Poll},
    };

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
    use tonic::transport::server::Connected;

    #[derive(Debug)]
    pub struct UnixStream(pub tokio::net::UnixStream);

    impl Connected for UnixStream {}

    impl AsyncRead for UnixStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            println!("READ...");
            Pin::new(&mut self.0).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for UnixStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            Pin::new(&mut self.0).poll_write(cx, buf)
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_flush(cx)
        }

        fn poll_shutdown(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_shutdown(cx)
        }
    }
}

#[cfg(not(unix))]
async fn unix_socket_server(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    panic!("Unix domain sockets only available on unix systems.");
    Ok(());
}