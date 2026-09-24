# builder stage
FROM rust:1.95.0 AS builder
WORKDIR /usr/src

# Copy root manifest and lock file first for dependency layer caching
COPY Cargo.toml Cargo.lock ./

# Copy all Cargo.toml files from workspace members explicitly to preserve directory structure
COPY domain/Cargo.toml domain/Cargo.toml
COPY infrastructure/sqlite/Cargo.toml infrastructure/sqlite/Cargo.toml
COPY infrastructure/vouch/Cargo.toml infrastructure/vouch/Cargo.toml
COPY application/api/Cargo.toml application/api/Cargo.toml

# Create stub source files so cargo can resolve the workspace and fetch/build dependencies
RUN mkdir -p domain/src infrastructure/sqlite/src infrastructure/vouch/src application/api/src && \
    echo "" > domain/src/lib.rs && \
    echo "" > infrastructure/sqlite/src/lib.rs && \
    echo "" > infrastructure/vouch/src/lib.rs && \
    echo "fn main() {}" > application/api/src/main.rs

# Build dependencies (this layer is cached unless Cargo.toml/Cargo.lock change)
RUN cargo build --release --package api

# Copy actual source code
COPY . .

# Rebuild with real sources (only our code recompiles, dependencies are cached)
# Touch all source files to ensure they're rebuilt with real code
RUN find . -name "*.rs" -not -path "./target/*" -exec touch {} + && \
    cargo build --release --package api

# runtime stage (distroless — minimal, production-focused)
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /usr/src/target/release/api /usr/local/bin/api
USER 65532
EXPOSE 8080
CMD ["/usr/local/bin/api"]
