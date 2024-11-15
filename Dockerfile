# Use an official Rust image as the build stage
FROM rust:1.72 AS builder

# Set the working directory
WORKDIR /app

# Copy the Cargo files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create an empty src directory (required by Cargo)
RUN mkdir src

# Pre-cache dependencies
RUN cargo build --release || true

# Now copy the actual source files
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use a more recent Debian version to ensure GLIBC compatibility
FROM debian:bookworm-slim

# Install required libraries
RUN apt-get update && apt-get install -y \
    libssl-dev \
 && rm -rf /var/lib/apt/lists/*

# Set the working directory in the runtime stage
WORKDIR /app

# Copy the compiled binary from the build stage
COPY --from=builder /app/target/release/simple_server /app/simple_server

# Set the application to run as a non-root user
RUN useradd -m rustuser
USER rustuser

# Expose the application port
EXPOSE 8000

# Run the application
CMD ["./simple_server"]
