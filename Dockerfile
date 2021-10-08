FROM rust as build

WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
    libpq-dev \
    libclang-dev \
    clang \
    libssl-dev \
    openssl \
    pkg-config \
    perl

RUN cd / && \
    USER=root cargo init --bin app && \
    rm /app/src/main.rs && \
    echo "fn main() {}" >> /app/src/main.rs

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY rust-toolchain rust-toolchain
RUN cargo fetch
RUN cargo build --release

COPY src src
COPY diesel.toml diesel.toml
RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

RUN apt-get update && apt-get install -y curl libpq-dev jq

COPY --from=build /app/target/release/indexer-explorer /usr/bin/indexer-explorer

COPY run.sh run.sh

ENTRYPOINT [ "./run.sh" ]
