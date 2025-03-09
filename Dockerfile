FROM rust:1-slim-bookworm as builder
WORKDIR /app
COPY ./ ./
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/ultimate_hacking_bot /usr/local/bin/
CMD ["ultimate_hacking_bot"]