use itertools::Itertools;
use std::time::{Duration, Instant};

pub fn fmt_ns(ts: u64) -> String {
    let seconds = ts / 1_000_000_000;
    let h = seconds / 3600;
    let m = (seconds - h * 3600) / 60;
    let s = seconds - 60 * m - h * 3600;

    let ns = ts % 1000;
    let mks = (ts % 1000_000 - ns) / 1000;
    let ms = (ts % 1000_000_000 - ns) / 1000_000;

    vec![(h, "h"), (m, "m"), (s, "s"), (ms, "ms"), (mks, "mks"), (ns, "ns")]
        .into_iter()
        .filter(|x| x.0 > 0)
        .map(|(v, u)| {
            if u == "ms" || u == "mks" || u == "ns" {
                format!("{:03}{}", v, u)
            } else {
                format!("{:02}{}", v, u)
            }
        })
        .join("")
}

#[derive(Debug, Clone)]
pub struct TimeProfiler {
    count: u64,
    values: Vec<u64>,
}

impl TimeProfiler {
    pub fn new() -> Self {
        Self { count: 0, values: Vec::new() }
    }

    pub fn values(&self) -> &[u64] {
        &self.values
    }

    pub fn add_duration(&mut self, duration: Duration) {
        self.add_value(duration.as_nanos() as u64);
    }

    pub fn add_value(&mut self, value: u64) {
        self.values.push(value);
    }

    pub fn measure(&mut self, now: Instant) -> u64 {
        let value = now.elapsed().as_nanos() as u64;
        self.add_value(value);
        value
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn total_ns(&self) -> u64 {
        self.values.iter().fold(0, |acc, item| acc + item)
    }

    pub fn avg(&self) -> u64 {
        if self.values.is_empty() {
            return 0;
        }
        self.total_ns() / self.values.len() as u64
    }

    pub fn prepare(&mut self) {
        self.values.sort();
    }

    pub fn median(&self) -> u64 {
        if self.values.is_empty() {
            return 0;
        }
        self.values[self.values.len() / 2].clone()
    }

    pub fn fmt_avg_ns(&self) -> String {
        fmt_ns(self.avg())
    }

    pub fn fmt_median_ns(&self) -> String {
        fmt_ns(self.median())
    }

    pub fn fmt_delim(&mut self, delimeter: &str, eq: &str) -> String {
        self.prepare();
        let total = self.total_ns();

        format!(
            "count{eq}{}{del}total{eq}{}{del}avg{eq}{}{del}median{eq}{}{del}throughput{eq}{}/s",
            self.values.len(),
            fmt_ns(total),
            self.fmt_avg_ns(),
            self.fmt_median_ns(),
            (self.values.len() as f64 * 1000_000_000.0 * 100.0 / total as f64).round() / 100.0,
            del = delimeter,
            eq = eq,
        )
    }

    pub fn fmt(&mut self) -> String {
        self.fmt_delim(" ", "=")
    }
}
