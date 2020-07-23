use futures::io::Error;
use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub fn tcp_server_std(addr: SocketAddr) {
    println!(
        "\n\
        RUN TCP SERVER: STD ==================\n\
        address: {}\n\
        ======================================\n",
        addr
    );

    let listener = TcpListener::bind(addr).expect("bind");

    for stream_res in listener.incoming() {
        let mut stream = stream_res.expect("new peer");
        let addr = stream.peer_addr().expect("addr");

        stream.set_ttl(1).expect("set ttl");
        stream.set_nodelay(true).expect("Can't set no_delay to true");
        stream.set_nonblocking(true).expect("Can't set channel to be non-blocking"); // THIS MAGICALLY INCREASE THE SPEED
        stream.set_read_timeout(None).expect("set timeout");
        stream.set_write_timeout(None).expect("set timeout");

        println!("{} new peer", addr);

        loop {
            let mut buf = [0; 4096];
            match stream.read(&mut buf) {
                Ok(0) => {
                    // - This reader has reached its "end of file" and will likely no longer be able to produce bytes. Note that this does not mean that the reader will always no longer be able to produce bytes.
                    // - The buffer specified was 0 bytes in length.
                    break;
                }
                Ok(n) => {
                    stream.write(&buf[0..n]).expect("sent");
                }
                Err(err) => match err.kind() {
                    ErrorKind::WouldBlock => {}
                    _ => {
                        eprintln!("{} receive_message: Error occurred while reading: {:?}", addr, err);
                        break;
                    }
                },
            }
        }

        println!("{} peer exit", addr);
    }
}
