FROM rust:1.52

WORKDIR /app
COPY . /app

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release

## Dockerfile.distroless
#
#ARG BASE_IMAGE=rust:1.52.1
#
#FROM $BASE_IMAGE as planner
#WORKDIR app
#RUN cargo install cargo-chef --version 0.1.20
#COPY . .
#RUN cargo chef prepare  --recipe-path recipe.json
#
#FROM $BASE_IMAGE as cacher
#WORKDIR app
#RUN cargo install diesel_cli --no-default-features --features postgres
#RUN cargo install cargo-chef --version 0.1.20
#COPY --from=planner /app/recipe.json recipe.json
#RUN cargo chef cook --release --recipe-path recipe.json
#
#FROM $BASE_IMAGE as builder
#WORKDIR app
#COPY . .
## Copy over the cached dependencies
#COPY --from=cacher /app/target target
#COPY --from=cacher $CARGO_HOME $CARGO_HOME
#RUN cargo build --release
#
#FROM gcr.io/distroless/cc-debian10
#COPY --from=builder /app/target/release/ohboi_backend /

#
# syntax=docker/dockerfile:1.2
#FROM rust:1.52 AS builder
#
#WORKDIR /app
#COPY . .
#RUN cargo install diesel_cli --no-default-features --features postgres
## Copy executable out of the cache so it is available in the final image.
#RUN cp target/debug/ohboi_backend ./ohboi_backend
##
#FROM rust:1.52
#COPY --from=builder /home/rust/src/ohboi_backend .
