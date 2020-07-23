use futures::io::Error;
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::str::from_utf8;

const SERVER: Token = Token(0);

fn handle_connection_read(stream: &mut TcpStream) -> io::Result<(bool, Vec<u8>)> {
    let mut received_data = Vec::with_capacity(4096);
    loop {
        let mut buf = [0; 256];
        match stream.read(&mut buf) {
            Ok(0) => {
                // Reading 0 bytes means the other side has closed the connection or is done writing, then so are we.
                return Ok((false, received_data));
            }
            Ok(n) => {
                // Would block "errors" are the OS's way of saying that the connection is not actually ready to perform this I/O operation.
                received_data.extend_from_slice(&buf[..n])
            }
            Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => break,
            Err(ref err) if err.kind() == io::ErrorKind::Interrupted => continue,
            Err(err) => return Err(err),
        }
    }

    return Ok((true, received_data));
}

fn handle_connection_write(stream: &mut TcpStream, data: Vec<u8>) -> io::Result<()> {
    match stream.write(data.as_slice()) {
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
            return handle_connection_write(stream, data);
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
    let mut unique_token = Token(SERVER.0);

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

                    let token = {
                        unique_token.0 += 1;
                        Token(unique_token.0)
                    };

                    stream.set_ttl(1).expect("set ttl");
                    stream.set_nodelay(true).expect("set_nodelay");

                    poll.registry().register(&mut stream, token, Interest::READABLE.add(Interest::WRITABLE)).expect("register");

                    connections.insert(token, stream);
                },
                token => {
                    if let Some(stream) = connections.get_mut(&token) {
                        let addr = stream.peer_addr().expect("peer addr");
                        match handle_connection_read(stream) {
                            Ok((true, buf)) => {
                                if let Err(err) = handle_connection_write(stream, buf) {
                                    println!("{} ERROR: {}", addr, err);
                                }
                            }
                            Ok((false, _)) => {
                                connections.remove(&token);
                                println!("{} peer exit", addr);
                            }
                            Err(err) => eprintln!("{} ERROR: {}", addr, err),
                        }
                    }
                }
            }
        }
    }
}
