FROM lukemathwalker/cargo-chef:0.1.50-rust-buster AS chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.* rust-toolchain.toml ./
COPY api api
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY Cargo.* rust-toolchain.toml ./
COPY api api
RUN cargo build --release --bin holaplex-hub-credentials


FROM debian:bullseye-slim as base
WORKDIR /app
RUN apt-get update -y && \
  apt-get install -y --no-install-recommends \
    ca-certificates \
    libpq5 \
    libssl1.1 \
  && \
  rm -rf /var/lib/apt/lists/*

RUN mkdir -p bin

COPY --from=builder /app/target/release/holaplex-hub-credentials bin
CMD ["bin/holaplex-hub-credentials"]

