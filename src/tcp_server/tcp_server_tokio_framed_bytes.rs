use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::TcpListener;
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
        if let Ok((stream, addr)) = listener.accept().await {
            println!("{} new peer", addr);
            stream.set_nodelay(true).expect("set_nodelay");
            stream.set_linger(None).expect("set_linger");

            let mut framed = Framed::new(stream, BytesCodec::new());
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
