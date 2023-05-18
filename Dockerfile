FROM rust:1.69.0-alpine3.17 as base
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN apk add --no-cache musl-dev && \
    cargo install cargo-chef && \
    rm -rf $CARGO_HOME/registry/
WORKDIR /app


FROM base as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM base as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zero-to-prod-rs


FROM alpine:3.17 as runtime
WORKDIR /app
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/zero-to-prod-rs zero-to-prod-rs
COPY configuration.yaml configuration.production.yaml ./
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "/app/zero-to-prod-rs" ]
