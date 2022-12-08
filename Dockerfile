FROM --platform=$BUILDPLATFORM rust:latest
WORKDIR /
COPY ./ ./
RUN cargo build --release


FROM --platform=$BUILDPLATFORM ubuntu:latest
COPY --from=0 /target/release ./
CMD ["./ultimate_hacking_bot"]