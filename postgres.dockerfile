# Use the official Rust image as the build stage
FROM messense/rust-musl-cross:x86_64-musl as builder

# Install the required updates & dependencies
# RUN apt-get update && \ 
#     apt install musl-gcc && \ 
#     apt install sqlite3 libsqlite-dev && \ 
#     rustup target add x86_64-unknown-linux-musl
# RUN cargo install diesel_cli --no-default-features --features "sqlite-bundled" 

# Create a new directory for the project
WORKDIR /app

# Copy the project files into the container
COPY . .

# Run migrations using diesel-cli
# RUN diesel migration run --database-url sqlite.db

# Build the Rust project with cargo-wasix
RUN cargo build --release --target x86_64-unknown-linux-musl

# Switch to a scratch image for the final image
FROM scratch as release

# App Directory
WORKDIR /app

# Copy only the necessary files from the build stage
# COPY --from=builder /app/sqlite.db ./sqlite.db
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/axum-api ./axum-api

# Set the entry point for the final image
CMD ["./axum-api"]

# Expose port
EXPOSE 5050
