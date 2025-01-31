# Stage 1: Build Stage
FROM rust:1.76-slim AS builder

WORKDIR /app

# Copy only the source files (excluding Cargo.lock)
COPY . .

# Delete the existing Cargo.lock file
RUN rm -f Cargo.lock

# Build the Rust application in release mode
RUN cargo build --release