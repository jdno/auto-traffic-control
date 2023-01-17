FROM rust:1 as builder

WORKDIR /usr/src/auto-traffic-control

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo install --path game/cli

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    dumb-init \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/cli /usr/local/bin/auto-traffic-control

ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["auto-traffic-control"]
