FROM rust:latest
WORKDIR /
COPY ./ ./
RUN cargo build --release

FROM ubuntu:latest
RUN apt update -y
RUN apt install libssl-dev
COPY --from=0 /target/release ./
CMD ["./ultimate_hacking_bot"]