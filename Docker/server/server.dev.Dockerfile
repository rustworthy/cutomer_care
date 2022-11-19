FROM rust:slim-buster

WORKDIR /app

COPY . .

RUN cargo build

CMD ["./target/debug/customer_care"]