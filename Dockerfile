FROM rust:1.90-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/alertmanager-silences-slack-reporter /usr/local/bin/

# Create non-root user
RUN useradd -m -u 1000 reporter
USER reporter

ENTRYPOINT ["alertmanager-silences-slack-reporter"]

