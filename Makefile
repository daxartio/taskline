DEFAULT_GOAL := all

.PHONY: all
all: fmt check test

.PHONY: check
check:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings

.PHONY: test
test:
	cargo test

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: d2
d2:
	d2 diagram.d2 diagram.png