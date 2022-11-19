FROM rust:latest as builder

ENV TARGET x86_64-unknown-linux-musl

RUN rustup target add ${TARGET}
RUN apt-get update -y 
RUN apt-get install -y musl-tools musl-dev build-essential gcc-x86-64-linux-gnu

WORKDIR /app

COPY . .

RUN cargo build --target ${TARGET} --release


FROM scratch

ENV TARGET x86_64-unknown-linux-musl

WORKDIR /app

COPY --from=builder /app/target/${TARGET}/release/customer_care ./

CMD ["./customer_care"]