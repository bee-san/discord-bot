# syntax = docker/dockerfile:experimental

FROM rust:latest 
WORKDIR /
COPY ./ ./
RUN --mount=type=tmpfs,target=/root/.cargo cargo build --release

FROM arm64v8/ubuntu:latest
COPY --from=0 /target/release ./
CMD ["./ultimate_hacking_bot"]