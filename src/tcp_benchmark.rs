use crate::time_profiler::TimeProfiler;
use hdrhist::HDRHist;
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct TcpBenchMark {
    addr: SocketAddr,
    no_delay: bool,
    non_blocking: bool,
}

impl TcpBenchMark {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            no_delay: true,
            non_blocking: true,
        }
    }

    pub fn run<TItem, FProcess, FPrepare, FEnd>(&self, buf: &mut [u8], items: Vec<TItem>, mut prepare: FPrepare, mut process: FProcess, mut end: FEnd)
    where
        FProcess: FnMut(&mut TcpStream, TItem, usize, &mut [u8]) -> (Duration, Duration),
        FPrepare: FnMut(&mut TcpStream),
        FEnd: FnMut(&mut TcpStream),
    {
        let len = items.len();
        let mut hist = HDRHist::new();
        let mut time_profiler = TimeProfiler::new();

        let mut sending_tp = TimeProfiler::new();
        let mut receive_tp = TimeProfiler::new();

        match TcpStream::connect(self.addr) {
            Ok(mut stream) => {
                prepare(&mut stream);

                println!(
                    "Connection to {} was established\
                    \n             stream ttl: {}\
                    \n         stream nodelay: {}\
                    \n    stream read_timeout: {:?}\
                    \n   stream write_timeout: {:?}",
                    stream.peer_addr().expect("addr"),
                    stream.nodelay().expect("nodealy"),
                    stream.ttl().expect("ttl"),
                    stream.read_timeout().expect("read_timeout"),
                    stream.write_timeout().expect("write_timeout"),
                );

                let half = len / 2;
                let step10 = len.clone() / 10;
                let step100 = len.clone() / 100;

                println!("\nReady to send {:#?} messages...", items.len());
                for (i, item) in items.into_iter().enumerate() {
                    let now = Instant::now();
                    // println!("sending {}...", i);
                    let (send_dur, receive_dur) = process(&mut stream, item, i, buf);
                    let val = time_profiler.measure(now);

                    sending_tp.add_duration(send_dur);
                    receive_tp.add_duration(receive_dur);

                    if i >= half {
                        hist.add_value(val);
                    }

                    if (i % step10) == 0 {
                        println!("sent {:>10} == {:>3}%", i, i / step100);
                    }
                }

                if (len % step10) == 0 {
                    println!("sent {:>10} == {:>3}%", len, len / step100);
                }

                end(&mut stream);

                stream.shutdown(Shutdown::Both).expect("shutdown call failed");
            }
            Err(err) => {
                println!("Couldn't connect to tcp_server. Error: {}", err);
            }
        }

        if !time_profiler.is_empty() {
            println!("-------------------------------------------------------------");
            println!("sending:\n    {}", sending_tp.fmt_delim("\n    ", ": "));
            println!("-------------------------------------------------------------");
            println!("receiving:\n    {}", receive_tp.fmt_delim("\n    ", ": "));
            println!("-------------------------------------------------------------");
            println!("Summary_string:\n{}", hist.summary_string());
            println!("-------------------------------------------------------------");
            println!("total:\n    {}", time_profiler.fmt_delim("\n    ", ": "));
            println!("-------------------------------------------------------------");

            // println!("Sent/received everything!");
            // println!("\n-------------------------------------------------------------\n");
            // println!("HDRHIST summary, measure in ns");
            // println!("\n-------------------------------------------------------------\n");
            // println!("summary:\n{:#?}", hist.summary().collect::<Vec<_>>());
            // println!("\n-------------------------------------------------------------\n");
            // println!("CDF summary:\n");
            // for entry in hist.ccdf_upper_bound() {
            //     println!("{:?}", entry);
            // }
        }
    }
}
