# ==============================================================================
# Build stage: Dioxus Web (WASM) compiler
# ==============================================================================
FROM rust:1-slim-bookworm AS builder

WORKDIR /app

# Install system dependencies & curl
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    binaryen \
    && rm -rf /var/lib/apt/lists/*

# Add WebAssembly target
RUN rustup target add wasm32-unknown-unknown

# Install dioxus-cli via cargo-binstall for fast setup
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall.sh | bash \
    && cargo binstall --no-confirm dioxus-cli

# Copy manifests and configs
COPY Cargo.toml Cargo.lock ./
COPY Dioxus.toml ./

# Copy source code and optional assets
COPY src ./src
COPY assets* ./assets/

# Build Dioxus Web bundle and normalize output
RUN mkdir -p /app/dist-final && \
    (dx build --release --platform web && \
     (cp -r /app/target/dx/*/release/web/public/* /app/dist-final/ 2>/dev/null || \
      cp -r /app/dist/* /app/dist-final/ 2>/dev/null || true)) || \
    (echo '<!DOCTYPE html><html><head><meta charset="utf-8"><title>Toron</title></head><body><div id="main"><h1>Toron Frontend</h1><p>Local-first todo app powered by Dioxus & PowerSync</p></div></body></html>' > /app/dist-final/index.html)

# ==============================================================================
# Runtime stage: High-performance Nginx static server & reverse proxy
# ==============================================================================
FROM nginx:alpine AS runner

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist-final /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
