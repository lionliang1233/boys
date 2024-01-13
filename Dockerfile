ARG RUST_VERSION=1.75
FROM rust:${RUST_VERSION} as builder

RUN \
    --mount=type=cache,target=/var/cache/apt \
    apt-get update \
    && apt-get install -y libgsl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-tarpaulin

WORKDIR /code/boys
COPY . .
RUN \
    --mount=type=cache,target=/code/boys/target \
    cargo tarpaulin --workspace --all-features --out Xml \
    && cargo build --release
