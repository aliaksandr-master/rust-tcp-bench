use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::SocketAddr;

const SERVER: Token = Token(0);

fn write(stream: &mut TcpStream, data: &[u8]) -> io::Result<()> {
    match stream.write(data) {
        Ok(n) if n < data.len() => {
            // We want to write the entire `data` buffer in a single go. If we write less we'll return a short write error (same as `io::Write::write_all` does).
            return Err(io::ErrorKind::WriteZero.into());
        }
        Ok(_) => {}
        Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
            // Would block "errors" are the OS's way of saying that the connection is not actually ready to perform this I/O operation.
        }
        Err(ref err) if err.kind() == io::ErrorKind::Interrupted => {
            // Got interrupted (how rude!), we'll try again.
            return write(stream, data);
        }
        Err(err) => return Err(err),
    }

    Ok(())
}

pub fn tcp_server_mio(server_addr: SocketAddr) {
    println!(
        "\n\
        RUN TCP SERVER: MIO ==================\n\
        address: {}\n\
        ======================================\n",
        server_addr
    );

    let mut poll = Poll::new().expect("poll");
    let mut events = Events::with_capacity(128);
    let mut server = TcpListener::bind(server_addr).expect("bind");

    poll.registry().register(&mut server, SERVER, Interest::READABLE).expect("register");

    let mut connections = HashMap::new();
    let mut unique_token = 1;

    loop {
        poll.poll(&mut events, None).expect("poll");

        for event in events.iter() {
            match event.token() {
                SERVER => loop {
                    let (mut stream, addr) = match server.accept() {
                        Ok(r) => r,
                        Err(e) => {
                            if e.kind() == io::ErrorKind::WouldBlock {
                                // If we get a `WouldBlock` error we know our listener has no more incoming connections queued, so we can return to polling and wait for some more.
                                break;
                            } else {
                                // If it was any other kind of error, something went wrong and we terminate with an error.
                                panic!("{}", e)
                            }
                        }
                    };

                    println!("{} new peer {:?}", addr, unique_token);

                    let token = Token(unique_token);
                    unique_token += 1;

                    stream.set_ttl(1).expect("set ttl");
                    stream.set_nodelay(true).expect("set_nodelay");
                    let addr = stream.peer_addr().expect("peer addr");

                    poll.registry().register(&mut stream, token, Interest::READABLE.add(Interest::WRITABLE)).expect("register");

                    connections.insert(token, (stream, [0; 8192], addr));
                },
                token => {
                    // println!("entr {:?}", token);
                    let mut must_remove = false;
                    let mut peer_addr: Option<SocketAddr> = None;
                    if let Some((stream, buf, addr)) = connections.get_mut(&token) {
                        peer_addr = Some(SocketAddr::clone(addr));
                        loop {
                            match stream.read(&mut buf[..]) {
                                Ok(0) => {
                                    // Rea ding 0 bytes means the other side has closed the connection or is done writing, then so are we.
                                    must_remove = true;
                                    println!("{}", 123);
                                    poll.registry().deregister(stream).expect("unregister");
                                    break;
                                }
                                Ok(n) => {
                                    if let Err(err) = write(stream, &buf[0..n]) {
                                        println!("{} ERROR: {}", addr, err);
                                        must_remove = true;
                                    }
                                    break;
                                }
                                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
                                    // If we get a `WouldBlock` error we know our listener has no more incoming connections queued, so we can return to polling and wait for some more.
                                    // must_remove = true;
                                    // println!("{}", 234);
                                    // poll.registry().deregister(stream).expect("unregister");
                                    // break;
                                }
                                Err(ref err) if err.kind() == io::ErrorKind::Interrupted => continue,
                                Err(err) => {
                                    eprintln!("{} ERROR: {}", addr, err);
                                    must_remove = true;
                                    break;
                                }
                            }
                        }
                        // println!("ext {}", must_remove);
                    }
                    if let Some(addr) = peer_addr {
                        if must_remove {
                            println!("{} peer exit", addr);
                            connections.remove(&token);
                        }
                    }
                }
            }
        }
    }
}
