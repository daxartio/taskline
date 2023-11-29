DEFAULT_GOAL := all

.PHONY: all
all: fmt check test docs

.PHONY: check
check:
	cargo fmt --all -- --check
	cargo clippy --all-features --all-targets -- -D warnings

.PHONY: test
test:
	cargo test --all-features

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: docs
docs:
	mdbook build

.PHONY: d2
d2:
	d2 diagram.d2 diagram.png
