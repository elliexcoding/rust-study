# Builder stage
FROM rust:1.76-slim-bookworm AS builder

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/app /usr/local/bin/app
CMD ["app"]