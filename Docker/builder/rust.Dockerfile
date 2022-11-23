FROM rust:slim-buster

RUN apt-get update -y 
RUN apt-get install -y musl-tools musl-dev build-essential gcc-x86-64-linux-gnu

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cargo-chef