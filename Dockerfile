FROM rust:alpine
RUN apk add build-base
WORKDIR /
COPY ./ ./
RUN cargo build --release

FROM rust:alpine
COPY --from=0 /target/release ./
CMD ["./ultimate_hacking_bot"]
