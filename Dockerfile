FROM rust:1.59.0 AS builder

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release

FROM rust:1.59.0 AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/rust_api rust_api

COPY configuration configuration

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./rust_api"]