all: fmt check test

check:
	cargo +nightly fmt --all -- --check
	cargo clippy --all-features --all-targets -- -D warnings

test:
	cargo test --all-features

fmt:
	cargo +nightly fmt --all
