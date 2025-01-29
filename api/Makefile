.PHONY: all format clean dev build test

all: format build test

format:
	command -v rustfmt >/dev/null || cargo install rustfmt
	cargo fmt -- --emit=files

clean:
	cargo clean

build:
	cargo lambda build --arm64 --release

dev:
	cargo lambda watch -w src -x run

prod: build
	./target/debug/api

test:
	cargo test