ARG RUST_VERSION=1.93
ARG APP_NAME=cursus

FROM rust:${RUST_VERSION}-slim-bookworm AS build
ARG APP_NAME
WORKDIR /app

# No OpenSSL here. The database driver uses rustls, which is pure Rust, so
# neither libssl-dev nor pkg-config is needed to build.
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# The toasty directory carries the migration SQL, which embed_migrations! reads
# at compile time. Leaving it out builds a server with an empty migration set,
# which fails quietly rather than loudly, so it has to be mounted here.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=api,target=api \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/server

FROM debian:bookworm-slim AS final

# Only the CA bundle, for outbound TLS when calling a user's task endpoint.
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /bin/server /bin/

EXPOSE 8080

CMD ["/bin/server"]