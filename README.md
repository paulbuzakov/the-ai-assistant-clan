# the-ai-assistant-clan

Collaborative hub for AI assistants, bots, and tools that work together as modular services.

## What this repository contains

- `src/shared`: shared Rust utilities (logging and environment settings)
- `src/external-bridges/telegram`: Telegram bridge service in Rust
- `docker/monitoring`: Grafana Alloy + Loki configuration
- `docker-compose.yml`: base services
- `docker-compose.monitoring.yaml`: observability stack
- `docker-compose.credentials.yaml`: local secrets override (kept out of git)

## Prerequisites

- Docker Desktop (or Docker Engine + Compose v2)
- `make`
- Rust toolchain (optional, only for local non-Docker development)

## Quick start (Docker)

1. Create or update credentials in `docker-compose.credentials.yaml`:

	```yaml
	services:
	  telegram:
		 environment:
			TELEGRAM_TOKEN: "your-telegram-bot-token"

	  grafana:
		 environment:
			- GF_SECURITY_ADMIN_PASSWORD=changeme
	```

2. Build and start all compose services:

	```bash
	make up
	```

3. Follow logs:

	```bash
	make logs
	```

4. Stop everything:

	```bash
	make down
	```

## Compose file loading behavior

The `Makefile` auto-loads `docker-compose.yml` plus any `docker-compose.*.yaml` / `docker-compose.*.yml` files in the project root.

That means:

- monitoring is included automatically via `docker-compose.monitoring.yaml`
- local secret overrides are included automatically via `docker-compose.credentials.yaml`

## Make targets

- `make build`: Build images for all compose services
- `make up`: Start services with build
- `make recreate`: Force recreate containers with build
- `make down`: Stop and remove containers
- `make logs`: Tail service logs
- `make shell`: Open shell in `telegram` container

## Services

- `telegram`: Rust bridge service (requires `TELEGRAM_TOKEN`)
- `alloy`: Collects and forwards telemetry
- `loki`: Log storage backend
- `grafana`: Dashboards and log exploration on `http://localhost:3000`

## Local Rust development (without Docker)

Run the Telegram bridge directly:

```bash
cd src/external-bridges/telegram
export TELEGRAM_TOKEN="your-telegram-bot-token"
cargo run
```

Run tests:

```bash
cd src/shared
cargo test
```

## Notes

- `TELEGRAM_TOKEN` is required; the Telegram service exits early when missing.
- `RUST_LOG` controls log verbosity (defaults to `info`).
