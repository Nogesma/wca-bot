FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY wcalive.graphql .
COPY src src/
COPY build.rs Cargo.toml Cargo.lock ./

RUN cargo install --path .

RUN ls /usr/local/cargo/bin/wca-bot

FROM debian:stable-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/wca-bot /usr/local/bin/wca-bot

CMD ["wca-bot"]
