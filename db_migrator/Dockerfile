# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.86.0
ARG APP_NAME=migrator_service


FROM rust:${RUST_VERSION}-alpine AS build
WORKDIR /app

RUN --mount=type=cache,target=/var/cache/apk \
    apk add clang lld musl-dev git

RUN --mount=type=bind,source=.,target=/app \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release -p migrator_service && \
    cp ./target/release/migrator_service /bin/server

FROM alpine:3.18 AS final

# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/
COPY db_migrator/migrations /migrations

CMD ["/bin/server"]
