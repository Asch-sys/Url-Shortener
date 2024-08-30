# Use the official Rust image as the base image
FROM rust:1.69 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin url_shortener
WORKDIR /url_shortener

# Copy the current directory contents into the container
COPY . .

# Build the project
RUN cargo build --release

# Use a smaller base image for the final stage
FROM debian:buster-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /url_shortener/target/release/url_shortener /usr/local/bin/url_shortener

# Run the binary
CMD ["url_shortener"]
