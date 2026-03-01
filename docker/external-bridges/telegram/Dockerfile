FROM rust:1.93.1 AS builder

WORKDIR /app
RUN apt-get update \
	&& apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
	&& rm -rf /var/lib/apt/lists/*

# Copy shared lib
COPY src/shared/src ./src/shared/src
COPY src/shared/Cargo.toml ./src/shared/Cargo.toml

# Copy the crate we want to build
COPY src/external-bridges/telegram/src ./src/external-bridges/telegram/src
COPY src/external-bridges/telegram/Cargo.toml ./src/external-bridges/telegram/Cargo.toml

# Build the binary in release mode from the crate directory
WORKDIR /app/src/external-bridges/telegram
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update \
	&& apt-get install -y --no-install-recommends ca-certificates libssl3 \
	&& rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/src/external-bridges/telegram/target/release/telegram /usr/local/bin/telegram

CMD ["telegram"]
