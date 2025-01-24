FROM rust:1.71.1 AS builder

RUN apt-get update && apt-get install -y \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo build --release

FROM alpine:latest

RUN apk update && apk add --no-cache ffmpeg

COPY --from=builder /usr/src/app/target/release/m3u8-flash /usr/local/bin/m3u8-flash

CMD ["m3u8-flash"]
