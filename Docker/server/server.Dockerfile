FROM rust:slim-buster as chef

RUN apt-get update -y 
RUN apt-get install -y musl-tools musl-dev build-essential gcc-x86-64-linux-gnu

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cargo-chef

################################################################################
FROM chef AS planner

COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

################################################################################
FROM chef as builder

COPY --from=planner recipe.json .
RUN cargo chef cook --target x86_64-unknown-linux-musl --release --recipe-path recipe.json

COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

################################################################################
FROM scratch as runtime

WORKDIR /app
COPY --from=builder /target/x86_64-unknown-linux-musl/release/customer_care .

CMD ["./customer_care"]