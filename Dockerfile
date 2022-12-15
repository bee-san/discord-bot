FROM rust:alpine as builder
LABEL maintainer="Autumn <https://github.com/bee-san>"
RUN apk add --no-cache build-base

# Encourage some layer caching here rather then copying entire directory that includes docs to builder container ~CMN
WORKDIR /
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo install --path .

FROM alpine:3.12
COPY --from=builder /usr/local/cargo/bin/ultimate_hacking_bot /usr/local/bin/ultimate_hacking_bot
ENTRYPOINT [ "/usr/local/bin/ultimate_hacking_bot" ]