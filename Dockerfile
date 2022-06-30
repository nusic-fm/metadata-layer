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
#  /app/target/release/node-template --dev --ws-external

COPY . .

RUN cargo build --release

LABEL org.opencontainers.image.source="https://github.com/nusic-fm/metadata-layer"

ENV environment dev
ENV place ws-external

# CMD ["/app/target/release/node-template"]

CMD ["sh", "-c", "/app/target/release/node-template --${environment} --${place}"]

EXPOSE 9944

# ENTRYPOINT ["/"]

# && ./target/release/node-template --dev --ws-external

# ENTRYPOINT ["tail", "-f", "/dev/null"]