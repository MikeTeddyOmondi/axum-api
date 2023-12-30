# Use the official Rust image as the build stage
FROM rust:1.74.0 as builder

# Install the required updates & dependencies
# RUN apt-get update
RUN rustup target add  x86_64-unknown-linux-musl

# Create a new directory for the project
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build the Rust project with cargo-wasix
RUN cargo build --release --target x86_64-unknown-linux-musl

# Switch to a scratch image for the final image
FROM scratch

# Copy only the necessary files from the build stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/axum-api ./axum-api

# Set the entry point for the final image
ENTRYPOINT ["axum-api"]

# Expose port
EXPOSE 5050

