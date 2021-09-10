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

# squid-deb-proxy is a tool to cache apt-get steps
# If host is running squid-deb-proxy on port 8000, populate /etc/apt/apt.conf.d/30proxy
# By default, squid-deb-proxy 403s unknown sources, so apt shouldn't proxy ppa.launchpad.net
RUN route -n | awk '/^0.0.0.0/ {print $2}' > /tmp/host_ip.txt
RUN echo "HEAD /" | nc `cat /tmp/host_ip.txt` 8000 | grep squid-deb-proxy \
  && (echo "Acquire::http::Proxy \"http://$(cat /tmp/host_ip.txt):8000\";" > /etc/apt/apt.conf.d/30proxy) \
  && (echo "Acquire::http::Proxy::ppa.launchpad.net DIRECT;" >> /etc/apt/apt.conf.d/30proxy) \
  || echo "No squid-deb-proxy detected on docker host"

# libpq-dev is a postgresl client
# ca-certificates are for ssl certificate resolution
# cron for queue producers
RUN apt-get update \
    && apt-get install -y libpq-dev ca-certificates cron
RUN apt-get remove -y aptitude aptitude-common mailutils mailutils-common mariadb-common mysql-common guile-2.2-libs:amd64
RUN rm -rf /var/lib/apt/lists/* \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && apt-get autoclean -y

WORKDIR /app
COPY run.sh .
COPY migrations ./migrations
COPY src/daemon/src/service/request/cache ./cache
# Copying only compiled binaries
COPY --from=builder /app/target/release/daemon ./daemon
COPY --from=builder /app/target/release/http ./http
# Copying diesel to have opportunity execute migrations before server start
COPY --from=cacher /usr/local/cargo/bin/diesel ./bin/diesel
