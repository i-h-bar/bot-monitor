FROM rust:1.88-bullseye as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo fetch
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/register-bot /app/register-bot

CMD ["/app/register-bot"]
