FROM rust:1.72-slim AS builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo install --path . --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rust-api-gateway /usr/local/bin/rust-api-gateway

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/rust-api-gateway"]
