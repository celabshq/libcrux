FROM rust:latest

# Install Valgrind, Clang, and libclang so bindgen can parse C headers
RUN apt-get update && apt-get install -y \
    valgrind \
    clang \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
