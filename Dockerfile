FROM --platform=$BUILDPLATFORM rust:latest
WORKDIR /
COPY ./ ./
RUN cargo build --release

FROM --platform=$BUILDPLATFORM debian:buster-slim
COPY --from=0 /target/release ./
CMD ["./ultimate_hacking_bot"]
