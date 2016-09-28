all: test

test:
	cargo build --verbose
	cargo test --verbose

bench:
	cargo bench --verbose
