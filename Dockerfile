FROM rust
WORKDIR /
COPY ./ ./
RUN cargo build --release

FROM rust
COPY --from=0 /target/release ./
CMD ["./ultimate_hacking_bot"]
