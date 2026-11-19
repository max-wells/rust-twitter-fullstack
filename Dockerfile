# Build stage: compile the Rust/Leptos app
FROM rust:nightly-bookworm AS builder

WORKDIR /app

# System dependencies: node + pnpm for Tailwind CSS, curl for cargo-leptos
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 20 LTS and pnpm
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y --no-install-recommends nodejs \
    && npm install -g pnpm@9 \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-leptos and the WASM target (pinned by rust-toolchain.toml)
RUN cargo install cargo-leptos \
    && rustup target add wasm32-unknown-unknown

# SQLx query macros are verified against the cached .sqlx/ metadata,
# so a live database is not required at build time.
ENV SQLX_OFFLINE=true

# Copy the full source and frontend assets
COPY . .

# Install Tailwind CSS deps and build the release app (wasm + site + server binary)
RUN pnpm install \
    && cargo leptos build --release

# Run stage: small image that only runs the compiled server binary
FROM debian:bookworm-slim AS runner

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server /app/server
COPY --from=builder /app/target/site /app/site

# Leptos configuration overrides (see Cargo.toml [workspace.metadata.leptos])
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV LEPTOS_SITE_ROOT=/app/site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_OUTPUT_NAME=twitter_clone_fullstack
ENV RUST_LOG=info

EXPOSE 3000

CMD ["./server"]
