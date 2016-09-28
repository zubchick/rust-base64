all: bench

test:
	cargo build --verbose
	cargo test --verbose

bench: test
	cargo bench --verbose
