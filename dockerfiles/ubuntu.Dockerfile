# Builder stage
FROM rust:latest as builder
WORKDIR /app

# Create a subdirectory in the container
RUN mkdir /app/project

# Copy the Rust project directory into /app/project
COPY . /app/project

# Build the project
RUN cargo build --release

# Final stage
FROM ubuntu:latest
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/project/target/release/rustEditor /app/

ENTRYPOINT ["./rustEditor"]
