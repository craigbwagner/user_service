# Use an official Rust image as the build stage
FROM rust:1.74 AS builder

WORKDIR /app
COPY . .

# Install dependencies and build
RUN cargo build --release

# Use a lightweight runtime image
FROM debian:bullseye-slim
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/user-service .

# Expose the service port
EXPOSE 8080

# Run the service
CMD ["./user-service"]
