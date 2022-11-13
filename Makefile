.PHONY: clean serve test devdb dev/create dev/drop

default: clean

clean:
	cargo fmt && cargo clippy

serve: clean
	fuser -k 7878/tcp || true && cargo run

test: clean
	cargo test

dev/create:
	docker-compose -f docker-compose.dev.yaml up -d --build

dev/drop:
	docker-compose -f docker-compose.dev.yaml down -v