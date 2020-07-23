#![allow(dead_code)]
#![deny(unused_must_use)]
mod tcp_bench;
mod tcp_benchmark;
mod tcp_server;
mod time_profiler;

use crate::tcp_server::tcp_server_mio::tcp_server_mio;
use crate::tcp_server::tcp_server_std::tcp_server_std;
use crate::tcp_server::tcp_server_tokio::tcp_server_tokio;
use crate::tcp_server::tcp_server_tokio_framed_bytes::tcp_server_tokio_framed_bytes;
use clap::{App, AppSettings, SubCommand};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tcp_bench::tcp_bench;

const CMD_TCP_SERVER_STD: &str = "tcp_server_std";
const CMD_TCP_SERVER_TOKIO: &str = "tcp_server_tokio";
const CMD_TCP_SERVER_TOKIO_FRAMED_BYTES: &str = "tcp_server_tokio_framed_bytes";
const CMD_TCP_SERVER_MIO: &str = "tcp_server_mio";

const CMD_BENCH: &str = "bench-tcp";

fn main() {
    let matches = App::new("rust-tcp-bench")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name(CMD_TCP_SERVER_STD))
        .subcommand(SubCommand::with_name(CMD_TCP_SERVER_TOKIO))
        .subcommand(SubCommand::with_name(CMD_TCP_SERVER_TOKIO_FRAMED_BYTES))
        .subcommand(SubCommand::with_name(CMD_TCP_SERVER_MIO))
        .subcommand(SubCommand::with_name(CMD_BENCH))
        .get_matches();

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9921);

    match matches.subcommand_name().unwrap() {
        CMD_TCP_SERVER_STD => tcp_server_std(socket),
        CMD_TCP_SERVER_TOKIO => tcp_server_tokio(socket),
        CMD_TCP_SERVER_TOKIO_FRAMED_BYTES => tcp_server_tokio_framed_bytes(socket),
        CMD_TCP_SERVER_MIO => tcp_server_mio(socket),

        CMD_BENCH => tcp_bench(socket, 100_000, 1024),
        _ => unreachable!(),
    }
}
