use std::env;
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
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let url=match args.pop() {
        Some(val) => val,
        None => panic!("Please specify an endpoint")
    };

    if url.starts_with("unix://") {
        unix_socket_server(url.strip_prefix("unix://").unwrap()).await?;
    } else if url.starts_with("tcp://") {
        tcp_server(url.strip_prefix("tcp://").unwrap()).await?;
    } else {
        panic!("Endpoint not supported");
    }

    Ok(())
}

async fn tcp_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
//    let addr = "[::1]:50051".parse()?;
    let address = addr.parse()?;
 
    Server::builder()
        .add_service(new_runtime_service_server())
        .add_service(new_image_service_server())
        .serve(address)
        .await?;

    Ok(())
}

#[cfg(unix)]
async fn unix_socket_server(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let cert = tokio::fs::read("data/server.pem").await?;
    let key = tokio::fs::read("data/server.key").await?;

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