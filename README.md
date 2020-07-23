# rust-tcp-bench

```bash
cargo run --release -- tcp_server_std

cargo run --release -- tcp_server_tokio

cargo run --release -- tcp_server_tokio_framed_bytes

cargo run --release -- tcp_server_mio

cargo run --release -- bench-tcp
```


## Results
```Intel Xeon 2678 v3```

### tcp_server_std
```
avg: 009mks313ns
median: 008mks965ns
throughput: 107369.23/s
```

### tcp_server_tokio
```
avg: 023mks017ns
median: 021mks289ns
throughput: 43444.58/s
```

### tcp_server_tokio_framed_bytes
```
avg: 021mks591ns
median: 019mks761ns
throughput: 46314.36/s
```

### tcp_server_mio
```
avg: 020mks444ns
median: 018mks624ns
throughput: 48912.13/s
```
