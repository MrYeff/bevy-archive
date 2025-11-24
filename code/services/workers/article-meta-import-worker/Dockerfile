# code/services/workers/article-meta-import-worker/Dockerfile
FROM rust:1.91 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY . .

# Build the worker binary
RUN cargo build --release -p article-meta-import-worker

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/article-meta-import-worker /usr/local/bin/article-meta-import-worker

CMD ["article-meta-import-worker"]
