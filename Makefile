.PHONY: help clean serve test dev/build dev/up-detached dev/up dev/down dev/drop prod/build ci/up-detached ci/down

default: clean

clean:
	cargo fmt && cargo clippy

serve: clean
	fuser -k 7878/tcp || true && cargo run

test:
	cargo test

builder:
	docker build -t rustworthy/rustbuilder . -f Docker/builder/rust.Dockerfile && docker push rustworthy/rustbuilder

# DEV
dev/build:
	docker-compose -f Docker/compose.dev.build.yaml build

dev/up-detached:
	docker-compose -f Docker/compose.dev.yaml up -d

dev/up:
	docker-compose -f Docker/compose.dev.yaml up

dev/down:
	docker-compose -f Docker/compose.dev.yaml down

dev/drop:
	docker-compose -f Docker/compose.dev.yaml down -v

# OPTIMIZED BINARY
prod/build:
	docker-compose -f Docker/compose.build.yaml build

# CI
ci/up-detached:
	docker-compose -f Docker/compose.ci.yaml up -d

ci/drop:
	docker-compose -f Docker/compose.ci.yaml down -v
