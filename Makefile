run:
	@docker compose up

build:
	@docker compose build

format:
	@cargo fmt

test:
	@cargo test -p api -- --test-threads=1