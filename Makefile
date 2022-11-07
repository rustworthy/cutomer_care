.PHONY: clean serve test devdb

default: clean


clean:
	cargo fmt && cargo clippy

serve: clean
	fuser -k 7878/tcp || true && cargo run

test: clean
	cargo test


db/create:
	docker-compose up -d --build db

db/drop:
	docker-compose down -v