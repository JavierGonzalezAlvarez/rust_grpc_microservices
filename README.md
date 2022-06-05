# Microservices with grpc in rust
docs:
https://crates.io/crates/tonic
https://crates.io/crates/tonic-build
https://crates.io/crates/prost

$ cargo new rust_grpc --bin

add rpc dependency for rust: tonic
add protocol buffer dependency for rust: prost
add tokio async runtime for rust: tokio

-to build protocol buffer during compilation
tonic-build = "0.7.2"

-to use tonic-build it we create a file called build.rs

-create a rpc server, rename main.rs to server.rs
-create a rpc ms client ...

## install protoc binary on linux
$ apt install libprotobuf-dev protobuf-compiler

## Run gRPC server on a terminal
$ cargo run --bin ms-server

## Run gRPC microservices client on a terminal
$ cargo run --bin msname-client
$ cargo run --bin mssurname-client