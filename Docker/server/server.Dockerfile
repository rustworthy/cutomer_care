FROM rustworthy/rustbuilder AS planner

WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM rustworthy/rustbuilder as builder

WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --target x86_64-unknown-linux-musl --release --recipe-path recipe.json

COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release --bin customer_care

RUN useradd -u 10002 customer_care


FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/customer_care .
COPY --from=builder /etc/passwd /etc/passwd

USER customer_care

CMD ["./customer_care"]