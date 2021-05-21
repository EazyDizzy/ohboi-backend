# syntax=docker/dockerfile:1.2
FROM ekidd/rust-musl-builder:stable AS builder

COPY . .
#RUN cargo install diesel_cli --no-default-features --features postgres
RUN --mount=type=cache,target=/home/rust/.cargo/git \
    --mount=type=cache,target=/home/rust/.cargo/registry \
    --mount=type=cache,sharing=private,target=/home/rust/src/target \
#    sudo chown -R rust: target /home/rust/.cargo && \
    cargo build && \
    # Copy executable out of the cache so it is available in the final image.
    cp target/x86_64-unknown-linux-musl/debug/ohboi_backend ./ohboi_backend

FROM rust:1.52
COPY --from=builder /home/rust/src/ohboi_backend .
