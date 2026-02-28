.PHONY: up build recreate down logs shell

COMPOSE_FILE := docker/docker-compose.yml
COMPOSE := docker compose -f $(COMPOSE_FILE)

build:
	$(COMPOSE) build

up:
	$(COMPOSE) up --build

recreate:
	$(COMPOSE) up --build --force-recreate

down:
	$(COMPOSE) down

logs:
	$(COMPOSE) logs -f

shell:
	$(COMPOSE) exec telegram /bin/sh
