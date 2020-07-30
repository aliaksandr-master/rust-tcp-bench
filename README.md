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
avg: 008mks987ns
median: 008mks557ns
throughput: 111261.03/s
```

### tcp_server_tokio
```
avg: 020mks242ns
median: 018mks537ns
throughput: 49402.06/s
```

### tcp_server_tokio_framed_bytes
```
avg: 021mks901ns
median: 019mks975ns
throughput: 45659.36/s
```

### tcp_server_mio
```
avg: 014mks278ns
median: 013mks316ns
throughput: 70034.03/s
```
