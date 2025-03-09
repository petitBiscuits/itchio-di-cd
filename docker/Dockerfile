# Use prebuilt Rust + WASM + Bevy image from GHCR
FROM ghcr.io/petitbiscuits/bevy-wasm-base:latest AS builder

# Set working directory
WORKDIR /app

# Copy only dependency files (so we don't invalidate the cache)
COPY Cargo.toml Cargo.lock ./

# Copy the actual source files
COPY src ./src

# ✅ Now build the project (dependencies are already cached!)
RUN cargo build --release --target wasm32-unknown-unknown

# Run wasm-bindgen to generate JavaScript bindings
RUN wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/*.wasm

# Second stage: Serve the WASM file with Nginx
FROM nginx:latest

# Copy Nginx configuration
COPY nginx/nginx.conf /etc/nginx/nginx.conf

# Copy the compiled WebAssembly artifacts to Nginx’s default web root
COPY nginx/index.html /usr/share/nginx/html/index.html
COPY --from=builder /app/out /usr/share/nginx/html/out

# Expose the correct port
EXPOSE 4000

# Start Nginx
CMD ["nginx", "-g", "daemon off;"]
