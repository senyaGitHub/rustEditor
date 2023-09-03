# Builder stage
FROM rust:latest as builder
WORKDIR /app

# Copy the entire Rust project directory into /app
COPY . /app

# Build the project
RUN cargo build --release

# Final stage
FROM ubuntu:latest
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/rustEditor /app/

ENTRYPOINT ["./rustEditor"]
