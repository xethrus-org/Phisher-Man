# Multi-stage build for smaller final image
FROM rust:1.91 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Create a dummy main.rs to build dependencies (cache layer)
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY static ./static

# Build the actual application
RUN touch src/main.rs && \
    cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install PostgreSQL client (for running migrations) and SSL certificates
RUN apt-get update && \
    apt-get install -y postgresql-client ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/phisherman .
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/static ./static

# Expose port
EXPOSE 3000

# Run the binary
CMD ["./phisherman"]
