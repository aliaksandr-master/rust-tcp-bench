use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buf = [0; 4 * 1024];
        match stream.read(&mut buf) {
            Ok(0) => {}
            Ok(n) => {
                stream.write(&buf[0..n]).expect("sent");
            }
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => {}
                _ => panic!("receive_message: Error occurred while reading: {:?}", err),
            },
        }
    }
}

pub fn tcp_blocking_server(addr: SocketAddr) {
    println!(
        "\n\
        RUN BLOCKING SERVER ==================\n\
        server address: {}\n\
        ======================================\n",
        addr
    );

    let listener = TcpListener::bind(addr).expect("bind");

    for stream_res in listener.incoming() {
        let mut stream = stream_res.expect("stream");
        stream.set_ttl(1).expect("set ttl");
        stream.set_nodelay(true).expect("Can't set no_delay to true");
        stream.set_nonblocking(true).expect("Can't set channel to be non-blocking"); // THIS MAGICALLY INCREASE THE SPEED
        stream.set_read_timeout(None).expect("set timeout");
        stream.set_write_timeout(None).expect("set timeout");
        handle_client(stream);
    }
}
