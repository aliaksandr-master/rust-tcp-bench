#![allow(dead_code)]
#![deny(unused_must_use)]
mod tcp_async_server;
mod tcp_async_server_framed;
mod tcp_bench;
mod tcp_benchmark;
mod tcp_blocking_server;
mod time_profiler;

use clap::{App, AppSettings, SubCommand};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tcp_async_server::tcp_async_server;
use tcp_async_server_framed::tcp_async_server_framed;
use tcp_bench::tcp_bench;
use tcp_blocking_server::tcp_blocking_server;

const CMD_BLOCKING_SERVER: &str = "blocking-tcp-server";
const CMD_ASYNC_SERVER: &str = "async-tcp-server";
const CMD_ASYNC_FRAMED_SERVER: &str = "async-tcp-server-framed";
const CMD_BENCH: &str = "bench-tcp";

fn main() {
    let matches = App::new("rust-tcp-bench")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name(CMD_BLOCKING_SERVER))
        .subcommand(SubCommand::with_name(CMD_ASYNC_SERVER))
        .subcommand(SubCommand::with_name(CMD_ASYNC_FRAMED_SERVER))
        .subcommand(SubCommand::with_name(CMD_BENCH))
        .get_matches();

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9921);

    match matches.subcommand_name().unwrap() {
        CMD_BLOCKING_SERVER => tcp_blocking_server(socket),
        CMD_ASYNC_SERVER => tcp_async_server(socket),
        CMD_ASYNC_FRAMED_SERVER => tcp_async_server_framed(socket),
        CMD_BENCH => tcp_bench(socket, 100_000, 1024),
        _ => unreachable!(),
    }
}
