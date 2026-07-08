
ARG RUST_VERSION=1.94
ARG APP_NAME=entities_service


FROM rust:${RUST_VERSION}-alpine AS build
WORKDIR /app

RUN --mount=type=cache,target=/var/cache/apk \
    apk add clang lld musl-dev git

RUN --mount=type=bind,source=.,target=/app \
    --mount=type=cache,target=/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release --target-dir /target -p entities_service && \
    cp /target/release/entities_service /bin/server

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

COPY --from=build /bin/server /bin/
EXPOSE 4000

CMD ["/bin/server"]
