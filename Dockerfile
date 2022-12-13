FROM --platform=linux/arm/v7 rust:latest as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
CMD ["cargo", "build", "--release"]

FROM --platform=linux/arm/v7 ubuntu:latest
WORKDIR /app

COPY --from=builder /app/target/release/ultimate_hacking_bot /app/ultimate_hacking_bot

CMD ["/app/ultimate_hacking_bot"]