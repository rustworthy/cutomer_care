FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build

CMD ["./target/debug/customer_care"]