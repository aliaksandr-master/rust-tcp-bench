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

    pub fn run<TItem, FProcess, FPrepare, FEnd>(
        &self,
        items: Vec<TItem>,
        mut prepare: FPrepare,
        mut process: FProcess,
        mut end: FEnd,
    ) where
        FProcess: FnMut(&mut TcpStream, TItem, usize) -> (Duration, Duration),
        FPrepare: FnMut(&mut TcpStream),
        FEnd: FnMut(&mut TcpStream),
    {
        let len = items.len();
        let mut hist = HDRHist::new();
        let mut time_profiler = TimeProfiler::new();

        let mut sending_tp = TimeProfiler::new();
        let mut receive_tp = TimeProfiler::new();

        println!("Connecting to the server {}...", self.addr);
        match TcpStream::connect(self.addr) {
            Ok(mut stream) => {
                // stream.set_linger(None).expect("set linger");

                println!(
                    "nodelay:{:?}  ::  read_timeout:{:?}  ::  write_timeout:{:?}",
                    stream.nodelay().expect("nodealy"),
                    stream.read_timeout().expect("read_timeout"),
                    stream.write_timeout().expect("write_timeout"),
                );

                core_affinity::set_for_current(core_affinity::get_core_ids().unwrap()[0]);
                println!("Connection established! Ready to send...");

                prepare(&mut stream);

                let half = len / 2;
                let step10 = len.clone() / 10;
                let step100 = len.clone() / 100;

                for (i, item) in items.into_iter().enumerate() {
                    let now = Instant::now();
                    let (send_dur, receive_dur) = process(&mut stream, item, i);
                    let val = time_profiler.measure(now);

                    sending_tp.add_duration(send_dur);
                    receive_tp.add_duration(receive_dur);

                    if i >= half {
                        hist.add_value(val);
                    }

                    if (i % step10) == 0 {
                        println!("{}%", i / step100);
                    }
                }
                if (len % step10) == 0 {
                    println!("{}%", len / step100);
                }

                end(&mut stream);

                stream
                    .shutdown(Shutdown::Both)
                    .expect("shutdown call failed");
            }
            Err(err) => {
                println!("Couldn't connect to server. Error: {}", err);
            }
        }

        if !time_profiler.is_empty() {
            println!("-------------------------------------------------------------");
            println!("total:\n    {}", time_profiler.fmt_delim("\n    ", ": "));
            println!("-------------------------------------------------------------");
            println!("sending:\n    {}", sending_tp.fmt_delim("\n    ", ": "));
            println!("-------------------------------------------------------------");
            println!("receiving:\n    {}", receive_tp.fmt_delim("\n    ", ": "));
            println!("-------------------------------------------------------------");
            println!("Summary_string:\n{}", hist.summary_string());
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
