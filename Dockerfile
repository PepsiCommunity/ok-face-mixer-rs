FROM rust:latest AS builder
WORKDIR /app

RUN cargo install trunk

COPY . .
RUN cargo build --bin ok-face-mixer-api --release

WORKDIR ok-face-mixer-web
RUN trunk build --release

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/res /app/res
COPY --from=builder /app/target/release/ok-face-mixer-api /app/ok-face-mixer-api
RUN chmod +x /app/ok-face-mixer-api

EXPOSE 8000
CMD ["/app/ok-face-mixer-api"]
