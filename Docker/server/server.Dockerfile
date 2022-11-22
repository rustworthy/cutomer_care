FROM rustworthy/rustbuilder AS planner

WORKDIR /app

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

################################################################################
FROM rustworthy/rustbuilder as builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --target x86_64-unknown-linux-musl --release --recipe-path recipe.json

COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release --bin customer_care

################################################################################
FROM scratch

WORKDIR /app
COPY --from=builder /target/x86_64-unknown-linux-musl/release/customer_care .

CMD ["./customer_care"]