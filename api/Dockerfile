FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# -------------------------
# Planner stage (cache-friendly)
# -------------------------
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json

# -------------------------
# Builder stage
# -------------------------
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Now copy the full source
COPY . .
RUN cargo build --release --bin api

# -------------------------
# Runtime stage
# -------------------------
FROM debian:trixie-slim AS runtime
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        openssl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/api /app/api
CMD ["/app/api"]
