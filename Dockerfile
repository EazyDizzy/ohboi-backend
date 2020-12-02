FROM rust:1.48

RUN apt-get update \
    && apt-get install -y postgresql \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . /app

RUN cargo install diesel_cli --no-default-features --features postgres
