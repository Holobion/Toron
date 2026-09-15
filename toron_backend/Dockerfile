# ==============================================================================
# Build stage
# ==============================================================================
FROM rust:1-slim-bookworm AS builder

WORKDIR /usr/src/toron_backend

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests and pre-fetch dependencies for layer caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copy real source code and build final binary
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ==============================================================================
# Runtime stage
# ==============================================================================
FROM debian:bookworm-slim AS runner

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 -U toron
USER toron

COPY --from=builder /usr/src/toron_backend/target/release/toron_backend /usr/local/bin/toron_backend

EXPOSE 3000

ENV RUST_LOG=info

CMD ["toron_backend"]
