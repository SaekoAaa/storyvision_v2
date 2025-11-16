# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.86.0
ARG APP_NAME=migrator_service


FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git

RUN --mount=type=bind,source=app,target=app \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=lib.rs,target=lib.rs \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/$APP_NAME /bin/server

FROM alpine:3.18 AS final

# Create a non-privileged user that the app will run under.
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

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/
# COPY config/docker_config.yaml /app/config.yaml
# COPY fixtures /fixtures
# ENV MIGRATIONS_PATH=./migrations
# Expose the port that the application listens on.
EXPOSE 1000

# What the container should run when it is started.
CMD ["/bin/server"]
