.PHONY: env env-down run
MAKEFLAGS += --silent
.DEFAULT_GOAL := run

run:
	cargo run

env:
	./scripts/oidctest-init.sh
	docker-compose build ipseity
	docker-compose up -d

env-down:
	docker-compose down