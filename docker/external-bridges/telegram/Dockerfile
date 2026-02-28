FROM rust:1.77-slim AS builder

WORKDIR /app
RUN apt-get update \
	&& apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
	&& rm -rf /var/lib/apt/lists/*

# Build the binary in release mode
COPY src/external-bridges/telegram/Cargo.toml ./Cargo.toml
COPY src/external-bridges/telegram/src ./src
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update \
	&& apt-get install -y --no-install-recommends ca-certificates libssl3 \
	&& rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/telegram /usr/local/bin/telegram

CMD ["telegram"]
