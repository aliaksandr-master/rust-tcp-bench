use bytes::BytesMut;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
pub async fn tcp_server_tokio(addr: SocketAddr) {
    println!(
        "\n\
        RUN TCP SERVER: TOKIO ===========\n\
        address: {}\n\
        =================================\n",
        addr
    );

    let mut listener = TcpListener::bind(addr).await.unwrap();
    loop {
        let (mut tcp_stream, addr) = listener.accept().await.expect("new peer");
        println!("{} new peer", addr);
        tcp_stream.set_nodelay(true).expect("set nodelay");
        tcp_stream.set_linger(None).expect("set linger");

        tokio::spawn(async move {
            let mut buf = [0; 8192];
            loop {
                match tcp_stream.read(&mut buf[..]).await {
                    Ok(0) => {
                        // - This reader has reached its "end of file" and will likely no longer be able to produce bytes. Note that this does not mean that the reader will always no longer be able to produce bytes.
                        // - The buffer specified was 0 bytes in length.
                        break;
                    }
                    Ok(n) => {
                        if let Err(err) = tcp_stream.write(&buf[0..n]).await {
                            eprintln!("{} ERROR: {}", addr, err);
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
        });
    }
}
