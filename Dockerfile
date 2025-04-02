FROM rust:latest AS builder
WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/res /app/res
COPY --from=builder /app/target/release/ok-face-mixer-rs /app/ok-face-mixer-rs
RUN chmod +x /app/ok-face-mixer-rs

EXPOSE 8000
CMD ["/app/ok-face-mixer-rs"]
