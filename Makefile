.PHONY: up build recreate down logs shell

COMPOSE_FILE := docker/docker-compose.yml
COMPOSE_CREDENTIALS_FILE := docker/docker-compose.credentials.yaml

COMPOSE_FILES := -f $(COMPOSE_FILE)
ifneq ("$(wildcard $(COMPOSE_CREDENTIALS_FILE))","")
	COMPOSE_FILES += -f $(COMPOSE_CREDENTIALS_FILE)
endif

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
