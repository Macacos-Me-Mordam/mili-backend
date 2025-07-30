FROM rustlang/rust:nightly as builder

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 ca-certificates && apt-get clean

COPY --from=builder /app/target/release/axum-api .
CMD ["./axum-api"]
