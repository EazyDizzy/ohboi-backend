ARG BASE_IMAGE=rust:1.52.1

# Computes the recipe file
FROM $BASE_IMAGE as planner
WORKDIR /app
RUN cargo install cargo-chef --version 0.1.21
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Caches dependencies
FROM $BASE_IMAGE as cacher
WORKDIR /app
RUN cargo install cargo-chef --version 0.1.21
COPY --from=planner /app/recipe.json recipe.json
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo chef cook --release --recipe-path recipe.json

# Builds the binary
FROM $BASE_IMAGE as builder
WORKDIR /app
COPY . .
# Copy compiled dependencies
COPY --from=cacher /app/target /app/target
# Copy cached dependencies
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release

FROM debian:buster-slim as runtime
WORKDIR /app
COPY run.sh .
COPY migrations ./migrations
# Copying only compiled binaries
COPY --from=builder /app/target/release/daemon ./daemon
COPY --from=builder /app/target/release/http ./http
# Copying diesel to have opportunity execute migrations before server start
COPY --from=cacher /usr/local/cargo/bin/diesel ./bin/diesel
# libpq-dev is a postgresl client
# ca-certificates are for ssl certificate resolution
# cron for queue producers
RUN apt-get update \
    && apt-get install -y libpq-dev ca-certificates cron \
    && rm -rf /var/lib/apt/lists/*