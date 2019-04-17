.PHONY: build env env-down
MAKEFLAGS += --silent
.DEFAULT_GOAL := build

build:
	cargo build -q

env:
	./scripts/oidctest-init.sh
	docker-compose build ipseity
	docker-compose up -d

env-down:
	docker-compose down