.PHONY: all format clean dev build test

all: format build test

format:
	command -v rustfmt >/dev/null || cargo install rustfmt
	cargo fmt -- --emit=files

clean:
	cargo clean

build:
	cargo build --release

dev:
	cargo watch -c -w src -x run

prod: build
	./target/debug/api

test:
	cargo test