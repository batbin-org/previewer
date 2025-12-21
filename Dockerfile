# Build stage
FROM rust:1.83-slim AS builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src to cache dependencies
RUN mkdir -p src assets && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

# Copy actual source
COPY src ./src
COPY assets ./assets
COPY TwoDark.tmTheme ./TwoDark.tmTheme
COPY editor.png ./editor.png

# Build release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/previewer_batbin /app/previewer

# Copy assets
RUN mkdir -p /data
COPY --from=builder /app/TwoDark.tmTheme /data/TwoDark.tmTheme
COPY --from=builder /app/editor.png /data/editor.png

# Environment variables
ENV PASTE_API_URL=https://umbra.batbin.me/api

EXPOSE 3030

ENTRYPOINT ["/app/previewer"]
CMD ["/data"]
