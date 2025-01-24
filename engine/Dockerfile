FROM rust:1.71.1 AS builder

RUN apt-get update && apt-get install -y \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release /usr/local/bin

RUN ln -s /usr/local/bin/m3u8-flash /usr/bin/m3u8-flash

CMD ["m3u8-flash"]
