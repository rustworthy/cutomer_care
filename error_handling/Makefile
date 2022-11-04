.PHONY: clean test
default: clean

clean:
	cargo fmt && cargo clippy

test: clean
	cargo test