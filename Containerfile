FROM rust:1.76 as builder

LABEL org.opencontainers.image.source=https://github.com/berquist/boys
LABEL org.opencontainers.image.description="build image for boys Rust crate"
LABEL org.opencontainers.image.licenses=GPL-3.0-only

# hadolint ignore=DL3008
RUN \
    --mount=type=cache,target=/var/cache/apt \
    apt-get update \
    && apt-get install -y --no-install-recommends \
      libgsl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-tarpaulin

WORKDIR /code/boys
COPY . .
RUN \
    --mount=type=cache,target=/code/boys/target \
    cargo tarpaulin --workspace --all-features --out Xml Html \
    && cargo build --release
