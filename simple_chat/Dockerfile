# Dockerfile

# Use the official Rust image as the base image
FROM rust:latest as builder

WORKDIR /usr/src/my_chat_app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src && touch src/lib.rs
RUN cargo build --release

# Copy the source code
COPY ./src ./src

# Build the application
RUN cargo install --path .

# Create the final image
FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/my_chat_app /usr/local/bin/my_chat_app

# Set the database URL as an environment variable
ENV DATABASE_URL="YOUR_DATABASE_URL"

CMD ["my_chat_app"]
