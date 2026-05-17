FROM rust:1.95-bullseye AS builder

RUN cargo install sqlx-cli --no-default-features --features rustls,postgres

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY migrations /migrations

CMD ["sh", "-c", "sqlx db create && sqlx migrate run --source /migrations"]
