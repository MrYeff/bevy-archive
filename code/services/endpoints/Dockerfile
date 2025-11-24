# code/services/endpoints/Dockerfile
FROM rust:1.91 as builder

WORKDIR /app

# Workspace root is the build context (code/)
COPY Cargo.toml Cargo.lock ./
COPY . .

# Build the endpoints binary (package "main")
RUN cargo build --release -p main

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Default binary name is the package name -> "main"
COPY --from=builder /app/target/release/main /usr/local/bin/main

EXPOSE 8080
CMD ["main"]