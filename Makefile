.PHONY: up build recreate down logs shell

COMPOSE_BASE := docker-compose.yml
COMPOSE_EXTRAS := $(sort $(wildcard docker-compose.*.yaml) $(wildcard docker-compose.*.yml))
COMPOSE_FILES := $(foreach f,$(COMPOSE_BASE) $(COMPOSE_EXTRAS),-f $(f))

COMPOSE := docker compose $(COMPOSE_FILES)

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
