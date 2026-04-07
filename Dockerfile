ARG RUST_VERSION=1.93
ARG APP_NAME=cursus

FROM rust:${RUST_VERSION}-slim-bookworm AS build
ARG APP_NAME
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=api,target=api \
    --mount=type=bind,source=migration,target=migration \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

FROM debian:bookworm-slim AS final

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /bin/server /bin/

EXPOSE 8080

CMD ["/bin/server"]