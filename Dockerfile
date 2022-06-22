# FROM rust:buster as builder
# WORKDIR /app

# RUN rustup default nightly-2022-02-19 && \
#   rustup target add wasm32-unknown-unknown --toolchain nightly-2022-02-19

# RUN apt-get update && \
#   apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
#   apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev

# COPY . .

# RUN cargo build --release && ./target/release/node-template --dev --ws-external

# ENTRYPOINT ["tail", "-f", "/dev/null"]

# # =============

FROM rustlang/rust:nightly

WORKDIR /app

RUN apt-get update -y && \
  apt-get install -y cmake pkg-config libssl-dev git gcc build-essential clang libclang-dev

RUN rustup target add wasm32-unknown-unknown

COPY . .

RUN cargo build --release 
# && ./target/release/node-template --dev --ws-external

# ENTRYPOINT ["tail", "-f", "/dev/null"]