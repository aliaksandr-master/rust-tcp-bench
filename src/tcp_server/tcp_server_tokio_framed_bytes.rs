use bytes::BytesMut;
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Error};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{BytesCodec, Framed};

#[tokio::main]
pub async fn tcp_server_tokio_framed_bytes(addr: SocketAddr) {
    println!(
        "\n\
        RUN TCP SERVER: TOKIO FRAMED BYTEs ===\n\
        address: {}\n\
        ======================================\n",
        addr
    );

    let mut listener = TcpListener::bind(addr).await.unwrap();
    loop {
        if let Ok((mut tcp_stream, addr)) = listener.accept().await {
            println!("{} new peer", addr);
            tcp_stream.set_nodelay(true).expect("set_nodelay");
            tcp_stream.set_linger(None).expect("set_linger");

            let mut framed = Framed::new(tcp_stream, BytesCodec::new());
            while let Some(msg_res) = framed.next().await {
                match msg_res {
                    Ok(msg) => {
                        if let Err(err) = framed.send(msg.freeze()).await {
                            eprint!("{} ERROR: {}", addr, err);
                            break;
                        }
                    }
                    Err(err) => {
                        eprint!("{} ERROR: {}", addr, err);
                        break;
                    }
                }
            }
            println!("{} peer exit", addr);
        } else {
            println!("peer connection error");
        }
    }
}
