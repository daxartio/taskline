.PHONY: check
check:
	cargo clippy --all-targets --all-features --workspace

.PHONY: test
test:
	cargo test --all-features --workspace

.PHONY: fmt
fmt:
	cargo fmt --all
