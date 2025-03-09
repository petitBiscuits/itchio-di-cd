# First stage: Build WASM binary
FROM rust:latest AS builder

# Install required tools (cached before copying source code)
RUN rustup target add wasm32-unknown-unknown && \
    cargo install wasm-bindgen-cli

# Set working directory
WORKDIR /app

# Copy only dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# ✅ Install dependencies *before* copying the source code
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# Now copy the actual source files
COPY src ./src

# Build the project
RUN cargo build --release --target wasm32-unknown-unknown

# Run wasm-bindgen to generate JavaScript bindings
RUN wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/*.wasm

# Second stage: Serve the WASM file with Nginx
FROM nginx:latest

COPY nginx/nginx.conf /etc/nginx/nginx.conf

# Copy the compiled WebAssembly artifacts to Nginx’s default web root

COPY nginx/index.html /usr/share/nginx/html

COPY --from=builder /app/out /usr/share/nginx/html/out

# Expose the web server port
EXPOSE 4000

# Start Nginx
CMD ["nginx", "-g", "daemon off;"]
