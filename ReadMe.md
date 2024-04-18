# A simple app to demo how to use grpc in rust project.

## Prepare
1. install rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. install protobuf
```bash
sudo apt update && sudo apt install protobuf-compiler -y
```


## Build and run
```bash
cargo build
cargo run server
cargo run client
```

## Reference
1. https://medium.com/@learnwithshobhit/comparing-capn-proto-and-grpc-in-rust-a-performance-and-feature-analysis-61d2da815d18
2. https://forge.rust-lang.org/infra/other-installation-methods.html#rustup
3. https://grpc.io/docs/protoc-installation/