FROM rust:1.58-slim

WORKDIR /usr/src/micropolis-rs/

COPY Cargo.toml \
    Cargo.lock \
    micropolis_core/Cargo.toml \
    micropolis_wasm/Cargo.toml \
    ./
COPY micropolis_core ./
RUN cargo build

COPY . .
