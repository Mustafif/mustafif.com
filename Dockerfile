# Use the official Rust nightly image as the base image
FROM rust:nightly AS builder

# Set the working directory
WORKDIR /home

# Copy the current directory contents into the container
COPY . .

# Build your Rust application in release mode
RUN cargo build --release

# Use a smaller base image for the final stage
FROM debian:buster-slim

# Copy the built application from the builder stage
COPY --from=builder /home/target/release/mustafif_com /usr/local/bin/mustafif_com

# Expose port 8000
EXPOSE 8000

# Set the entry point to run your application
CMD ["mustafif_com"]
