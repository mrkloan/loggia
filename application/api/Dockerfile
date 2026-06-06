# builder stage
FROM rust:1.95.0 AS builder
WORKDIR /usr/src

# Copy root manifest and lock file first for dependency layer caching
COPY Cargo.toml Cargo.lock ./

# Copy all Cargo.toml files from workspace members
# Using wildcard to ensure any new crate added will be included
COPY */Cargo.toml ./

# Create stub source files for all crates with Cargo.toml
# This dynamically finds all Cargo.toml files (except root) and creates appropriate stubs
RUN find . -maxdepth 2 -name "Cargo.toml" ! -path "./Cargo.toml" -exec dirname {} \; | while read -r crate_dir; do \
    src_dir="$crate_dir/src"; \
    mkdir -p "$src_dir"; \
    # Check if the crate path contains "api" (binary crate)
    if echo "$crate_dir" | grep -q "api"; then \
      echo "fn main() {}" > "$src_dir/main.rs"; \
    else \
      echo "" > "$src_dir/lib.rs"; \
    fi; \
done

# Build all workspace dependencies (this layer is cached unless Cargo.toml/Cargo.lock change)
# Using --workspace ensures all current and future workspace members are built
RUN cargo build --release --workspace

# Copy actual source code
COPY . .

# Rebuild with real sources (only our code recompiles, dependencies are cached)
# Touch all source files to ensure they're rebuilt with real code
RUN find . -name "*.rs" -not -path "./target/*" -exec touch {} + && \
    cargo build --release --workspace

# runtime stage (distroless — minimal, production-focused)
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /usr/src/target/release/api /usr/local/bin/api
USER 65532
EXPOSE 8080
CMD ["/usr/local/bin/api"]
