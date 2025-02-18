.PHONY: all format clean dev build test

all: format build test

format:
	command -v rustfmt >/dev/null || cargo install rustfmt
	cargo fmt -- --emit=files

clean:
	cargo clean

dev:
	cargo lambda watch -w src -x run