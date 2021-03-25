FROM rust:1.49

RUN apt update \
    && apt-get update \
    && apt-get install -y postgresql \
    && apt-get install -y cmake \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . /app

RUN cargo install diesel_cli --no-default-features --features postgres
