.PHONY: env env-down
MAKEFLAGS += --silent

default:
	echo "No default target"

env:
	./scripts/oidctest-init.sh
	docker-compose build ipseity
	docker-compose up -d

env-down:
	docker-compose down