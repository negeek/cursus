run:
	@docker compose up

build:
	@docker compose build

format:
	@cargo fmt

test:
	@cargo test -- --test-threads=1