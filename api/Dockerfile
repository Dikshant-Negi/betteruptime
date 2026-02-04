FROM rust:latest AS builder
WORKDIR /app
ENV SQLX_OFFLINE=true

# Workspace manifests
COPY Cargo.toml Cargo.lock ./
COPY .sqlx .sqlx
COPY api/Cargo.toml api/
COPY worker/Cargo.toml worker/
COPY redis_stream/Cargo.toml redis_stream/
COPY store/Cargo.toml store/

# Dummy source ONLY for dependency caching
RUN mkdir -p api/src worker/src redis_stream/src store/src && \
    echo "fn main(){}" > api/src/main.rs && \
    echo "pub fn dummy(){}" > redis_stream/src/lib.rs && \
    echo "pub fn dummy(){}" > store/src/lib.rs

#ONLY fetch deps, DO NOT build
RUN cargo fetch

# Remove dummy source
RUN rm -rf api/src worker/src redis_stream/src store/src

# Real source
COPY api api
COPY worker worker
COPY redis_stream redis_stream
COPY store store

#Build ONLY real code
RUN cargo build --release -p api

# Runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api /usr/local/bin/api
EXPOSE 3000
CMD ["/usr/local/bin/api"]
