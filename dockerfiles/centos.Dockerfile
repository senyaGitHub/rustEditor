FROM rust:latest as builder
WORKDIR /app
COPY ../ /app
RUN cargo build --release

FROM ubuntu:latest
WORKDIR /app
COPY --from=builder /app/target/release/rustEditor /app/
ENTRYPOINT ["./rustEditor"]