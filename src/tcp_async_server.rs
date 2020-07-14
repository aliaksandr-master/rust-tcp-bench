use bytes::BytesMut;
use futures::StreamExt;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Error};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
pub async fn tcp_async_server(addr: SocketAddr) {
    println!(
        "\n\
        RUN ASYNC SERVER ==================\n\
        server address: {}\n\
        ======================================\n",
        addr
    );

    let mut listener = TcpListener::bind(addr).await.unwrap();
    loop {
        match listener.accept().await {
            Ok((mut tcp_stream, addr)) => {
                loop {
                    let mut buf = BytesMut::with_capacity(4096);
                    loop {
                        match tcp_stream.read_buf(&mut buf).await {
                            Ok(0) => {}
                            Ok(n) => {
                                //
                                tcp_stream.write(&buf[0..n]).await.expect("sent");
                                break;
                            }
                            Err(err) => {
                                //
                                panic!("{}", err)
                            }
                        }
                    }
                }
            }
            Err(err) => {
                //
                panic!("{}", err);
            }
        }
    }
}
