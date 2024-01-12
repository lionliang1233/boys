ARG RUST_VERSION=1.75
FROM rust:${RUST_VERSION} as builder

RUN apt-get update \
    && apt-get install -y libgsl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /code/boys
COPY . .
RUN cargo install cargo-tarpaulin \
    && cargo test
