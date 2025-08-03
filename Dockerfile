FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY migration ./migration

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

RUN rm -rf src/
COPY src ./src

RUN cargo build --release


FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl curl && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/axum-api /usr/local/bin/axum-api

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

EXPOSE 3000

ENV PORT=3000

CMD ["axum-api"]