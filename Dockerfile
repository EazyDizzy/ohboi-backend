FROM rust:1.49

RUN apt-get update \
    && apt-get install -y postgresql \
    && apt-get install -y cmake \
#    && apt-get install -y pthreads \
#    && apt-get install -y zlib \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . /app

RUN cargo install diesel_cli --no-default-features --features postgres
