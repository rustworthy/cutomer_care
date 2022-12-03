FROM rust:slim-buster

RUN apt-get update -y 
RUN apt-get install -y musl-tools

RUN rustup target add x86_64-unknown-linux-musl

RUN rustup toolchain install nightly
RUN rustup target add --toolchain nightly x86_64-unknown-linux-musl

RUN cargo install cargo-chef
