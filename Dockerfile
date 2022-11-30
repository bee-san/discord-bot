FROM rust:alpine as builder
LABEL maintainer="Autumn <https://github.com/bee-san>"
RUN apk add --no-cache build-base

WORKDIR /usr/src/discord_bot
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo install --path .

COPY target/release/ultimate_hacking_bot ./

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/ultimate_hacking_bot /usr/local/bin/ultimate_hacking_bot
ENTRYPOINT [ "/usr/local/bin/ultimate_hacking_bot" ]