# Use a minimal Alpine Linux image as the base
FROM alpine:latest

# Set the working directory
WORKDIR /app

# Copy the Rust application into the image
COPY . .

# Build the Rust application
RUN apk add --no-cache rust && \
    cargo build --release

# Set the command to run when the container starts
CMD ["./target/release/hackweek-22"]
