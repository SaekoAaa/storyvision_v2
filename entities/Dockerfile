


ARG RUST_VERSION=1.86.0
ARG APP_NAME=entities_service


FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git

RUN --mount=type=bind,source=app,target=app \
    --mount=type=bind,source=features,target=features \
    --mount=type=bind,source=model,target=model\
    --mount=type=bind,source=features,target=features\
    --mount=type=bind,source=utils,target=utils\
    --mount=type=bind,source=constants.rs,target=constants.rs\
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=lib.rs,target=lib.rs \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/$APP_NAME /bin/server

FROM alpine:3.18 AS final

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
EXPOSE 4000

# What the container should run when it is started.
CMD ["/bin/server"]
