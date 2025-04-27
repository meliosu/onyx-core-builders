FROM rust:1.86-slim AS builder

WORKDIR /build

COPY Cargo.* .

RUN mkdir src                            \
    && echo "fn main() {}" > src/main.rs \
    && echo "fn dummy() {}" > src/lib.rs \
    && cargo build --release             \
    && rm -rf src

COPY src src
COPY migrations migrations
COPY templates templates

RUN touch src/main.rs src/lib.rs \
    && cargo build --release

FROM ubuntu:24.04

WORKDIR /app

COPY --from=builder /build/target/release/onyx-core-builders .

ENTRYPOINT ./onyx-core-builders


