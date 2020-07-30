# rust-tcp-bench

```bash
cargo run --release -- tcp_server_std

cargo run --release -- tcp_server_tokio

cargo run --release -- tcp_server_tokio_framed_bytes

cargo run --release -- tcp_server_mio

cargo run --release -- bench-tcp
```

## Results
```
Intel Xeon 2678 v3
on localhost

messages count = 100000
```

### tcp_server_std
```
avg: 009mks313ns
median: 008mks965ns
throughput: 107369.23/s
```

### tcp_server_tokio
```
avg: 022mks284ns
median: 020mks563ns
throughput: 44874.85/s
```

### tcp_server_tokio_framed_bytes
```
avg: 022mks529ns
median: 020mks847ns
throughput: 44385.42/s
```

### tcp_server_mio
```
avg: 015mks147ns
median: 013mks871ns
throughput: 66018.73/s
```
