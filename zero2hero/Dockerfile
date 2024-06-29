# Builder stage
FROM rust:1.76-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev
ENV SQLX_OFFLINE=true
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/zero2hero /usr/local/bin/zero2hero
COPY --from=builder /usr/src/app/config/docker-config.yaml /etc/zero2hero/
