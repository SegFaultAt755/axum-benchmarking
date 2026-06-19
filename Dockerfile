FROM rust:1.95 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runner
WORKDIR /app

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/axum-benchmarking .

EXPOSE 8000
CMD ["./axum-benchmarking"]
