use crate::tcp_benchmark::TcpBenchMark;
use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::{Duration, Instant};

fn send_msg_and_waiting_for_any_response(stream: &mut TcpStream, msg: Vec<u8>) -> (Duration, Duration) {
    let now = Instant::now();
    stream.write(&msg).expect("sent");
    let send_dur = now.elapsed();

    let now = Instant::now();
    let mut buf = [0; 4 * 1024];

    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                // println!("read!  0");
            }
            Ok(n) => {
                // assert_eq!(msg.len(), n);
                // for (i, b) in msg.into_iter().enumerate() {
                //     assert_eq!(b, buf[i])
                // }
                // println!("read!");
                return (send_dur, now.elapsed());
            }
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => {
                    // println!("read!  would-block");
                }
                _ => panic!("receive_message: Error occurred while reading: {:?}", err),
            },
        }
    }
}

pub fn tcp_bench(addr: SocketAddr, items_count: usize, size: usize) {
    println!(
        "\n\
        RUN TESTING ==================\n\
        server address: {}\n\
        message size:   {}bytes\n\
        ==============================\n",
        addr, size
    );

    let items = (0..items_count)
        .map(|_| {
            let mut buf = Vec::new();
            for _i in 0..size {
                let n: u8 = rand::random();
                buf.push(n);
            }
            buf
        })
        .collect::<Vec<_>>();

    TcpBenchMark::new(addr).run(
        items,
        |stream| {
            stream.set_ttl(1).expect("set ttl");
            stream.set_nodelay(true).expect("Can't set no_delay to true");
            stream.set_nonblocking(true).expect("Can't set channel to be non-blocking"); // THIS MAGICALLY INCREASE THE SPEED
            stream.set_read_timeout(None).expect("set timeout");
            stream.set_write_timeout(None).expect("set timeout");
        },
        |stream, msg, _i| send_msg_and_waiting_for_any_response(stream, msg),
        |_stream| {},
    );
}
