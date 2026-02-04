# ---------- Build stage ----------
FROM rust:latest AS builder
WORKDIR /app
ENV SQLX_OFFLINE=true

# Workspace manifests
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/
COPY worker/Cargo.toml worker/
COPY redis_stream/Cargo.toml redis_stream/
COPY store/Cargo.toml store/
COPY .sqlx .sqlx

# Dummy sources (for caching)
RUN mkdir -p api/src worker/src/bin redis_stream/src store/src && \
    echo "fn main() {}" > api/src/main.rs && \
    echo "fn main() {}" > worker/src/bin/consumer.rs && \
    echo "fn main() {}" > worker/src/bin/publisher.rs && \
    echo "fn main() {}" > worker/src/bin/notifier.rs && \
    echo "pub fn dummy() {}" > redis_stream/src/lib.rs && \
    echo "pub fn dummy() {}" > store/src/lib.rs

RUN cargo fetch

# Remove dummy
RUN rm -rf api/src worker/src redis_stream/src store/src

# Real source
COPY api api
COPY worker worker
COPY redis_stream redis_stream
COPY store store

#Build ALL worker binaries
RUN cargo build --release -p worker --bins


# ---------- Runtime stage ----------
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

#Copy binaries
COPY --from=builder /app/target/release/consumer /usr/local/bin/consumer
COPY --from=builder /app/target/release/publisher /usr/local/bin/publisher
COPY --from=builder /app/target/release/notifier /usr/local/bin/notifier
