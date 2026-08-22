.PHONY: run build format lint check test

run:
	@docker compose up

build:
	@docker compose build

format:
	@cargo fmt --all

lint:
	@cargo clippy --workspace --all-targets -- -D warnings
	@cargo fmt --all -- --check

check:
	@cargo check --workspace --all-targets

test:
	@cargo test -p api
