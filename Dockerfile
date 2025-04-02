FROM rust:latest AS builder
WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/res /app/res
COPY --from=builder /app/target/release/ok-face-mixer-api /app/ok-face-mixer-api
RUN chmod +x /app/ok-face-mixer-api

EXPOSE 8000
CMD ["/app/ok-face-mixer-api"]
