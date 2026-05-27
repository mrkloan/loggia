# builder stage
FROM rust:1.95.0 AS builder
WORKDIR /usr/src

# Copy manifests and lock file first for dependency layer caching
COPY Cargo.toml Cargo.lock ./
COPY domain/Cargo.toml domain/Cargo.toml
COPY infrastructure/sqlite/Cargo.toml infrastructure/sqlite/Cargo.toml
COPY application/api/Cargo.toml application/api/Cargo.toml

# Create stub source files so cargo can resolve the workspace and fetch/build dependencies
RUN mkdir -p domain/src infrastructure/sqlite/src application/api/src && \
    echo "" > domain/src/lib.rs && \
    echo "" > infrastructure/sqlite/src/lib.rs && \
    echo "fn main() {}" > application/api/src/main.rs

# Build dependencies only (this layer is cached unless Cargo.toml/Cargo.lock change)
RUN cargo build --release --package api

# Copy actual source code
COPY . .

# Rebuild with real sources (only our code recompiles, dependencies are cached)
RUN touch domain/src/lib.rs infrastructure/sqlite/src/lib.rs application/api/src/main.rs && \
    cargo build --release --package api

# runtime stage (distroless — minimal, production-focused)
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /usr/src/target/release/api /usr/local/bin/api
USER 65532
EXPOSE 8080
CMD ["/usr/local/bin/api"]
