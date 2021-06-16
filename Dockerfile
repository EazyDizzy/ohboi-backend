ARG BASE_IMAGE=rust:1.52.1

FROM lukemathwalker/cargo-chef as planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo chef cook --release --recipe-path recipe.json

FROM $BASE_IMAGE as builder
WORKDIR /app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target /app/target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release

FROM $BASE_IMAGE as runtime
WORKDIR /app
COPY --from=builder /app /app
COPY --from=builder $CARGO_HOME $CARGO_HOME