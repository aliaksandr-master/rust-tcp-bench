use bytes::BytesMut;
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Error};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{BytesCodec, Framed};

#[tokio::main]
pub async fn tcp_async_server_framed(addr: SocketAddr) {
    println!(
        "\n\
        RUN ASYNC FRAMED SERVER ==============\n\
        server address: {}\n\
        ======================================\n",
        addr
    );

    let mut listener = TcpListener::bind(addr).await.unwrap();
    loop {
        match listener.accept().await {
            Ok((mut tcp_stream, addr)) => {
                let mut framed = Framed::new(tcp_stream, BytesCodec::new());
                while let Some(msg_res) = framed.next().await {
                    match msg_res {
                        Ok(msg) => {
                            framed.send(msg.freeze()).await.expect("sent");
                        }
                        Err(err) => {
                            panic!("{}", err);
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
