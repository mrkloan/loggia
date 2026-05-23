# Hexagonal Architecture Workspace Specification

This specification outlines the architectural standards, directory layout, dependency rules, and contribution guidelines for the `loggia` Rust project. It serves as the single source of truth for the codebase's skeleton and future feature development.

---

## 1. Architectural Philosophy

We use a consolidated **Hexagonal Architecture (Ports & Adapters)** designed for extreme modularity, testability, and future scalability. The architecture enforces a strict **Dependency Rule**: *dependencies must only point inward toward the core business logic.*

```
                 [ DRIVING ADAPTERS ]
                     (Inbound)
             ┌─────────────────────────┐
             │     application/api     │ (Axum REST API)
             └────────────┬────────────┘
                          │
                          ▼ (Calls Inbound Ports)
  ┌────────────────────────────────────────────────────────┐
  │ CORE DOMAIN           │                                │
  │                       ▼                                │
  │             ┌───────────────────┐                      │
  │             │   Inbound Ports   │ (Use Case Traits)    │
  │             └─────────┬─────────┘                      │
  │                       │                                │
  │                       ▼                                │
  │             ┌───────────────────┐                      │
  │             │   App Services    │ (Coordinates Logic)  │
  │             └─────────┬─────────┘                      │
  │                       │                                │
  │                       ▼                                │
  │             ┌───────────────────┐                      │
  │             │   Domain Models   │ (Entities & Rules)   │
  │             └─────────┬─────────┘                      │
  │                       │                                │
  │                       ▼                                │
  │             ┌───────────────────┐                      │
  │             │  Outbound Ports   │ (Repository Traits)  │
  │             └─────────┬─────────┘                      │
  └───────────────────────┼────────────────────────────────┘
                          │
                          ▼ (Implemented by Outbound Adapter)
             ┌─────────────────────────┐
             │    infrastructure/db    │ (SQLite / SQLx)
             └─────────────────────────┘
                 [ DRIVEN ADAPTERS ]
                     (Outbound)
```

### The Symmetrical Pillars
1. **Core (`domain`)**: The inner hexagon. Contains pure business logic, entity definitions, domain errors, and interfaces (Ports).
2. **Infrastructure (`infrastructure/`)**: Pluggable driven adapters (outbound). These adapt third-party technologies (databases, external mail systems, file systems) to implement the ports defined by `domain`.
3. **Application (`application/`)**: Concrete driving adapters (inbound) and executables. They act as composition roots, parse external delivery inputs (e.g. JSON requests, command-line arguments), and wire dependencies together.

---

## 2. Directory & Workspace Layout

The project is managed as a multi-crate **Cargo Workspace** to enforce strict compile-time boundaries between layers.

```
loggia/
├── Cargo.toml                          # Workspace root (registers all crates)
├── domain/                             # Crate (Library): Business logic, Ports, and Services
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── errors.rs                   # Cross-cutting domain error types
│       ├── health/                     # Health capability
│       │   ├── mod.rs
│       │   ├── system_health.rs        # SystemHealth entity
│       │   └── check_health.rs         # CheckHealthUseCase + HealthRepository + HealthService
│       └── identity/                   # Identity capability
│           └── mod.rs                  # User entity
├── infrastructure/                     # Outbound Adapters Group
│   └── sqlite/                         # Crate (Library): SQLite database repository (SQLx)
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs                  # SQLx pools, configuration, and migrations
│           ├── migrations/             # SQL migration scripts (.sql)
│           └── health/                 # Health capability adapter
│               └── check_health.rs     # SqliteHealthRepository
└── application/                        # Inbound Delivery & Executables Group
    └── api/                            # Crate (Binary): HTTP Web Server (Axum)
        ├── Cargo.toml
        └── src/
            ├── main.rs                 # HTTP Composition Root & Dependency Injection
            └── http/                   # HTTP Routing
                ├── mod.rs              # Router composition
                ├── health/             # Health capability
                │   └── check_health.rs # GET /health handler
                └── identity/           # Identity capability
                    ├── authenticate.rs # AuthenticatedUser extractor (reusable)
                    └── get_me.rs       # GET /me handler
```

---

## 3. Strict Compile-Time Dependency Rules

These rules are enforced by Cargo's compilation pipeline. A violation will result in a compile error, preventing architectural drift.

```
                  ┌──────────────────────────────────────────────┐
                  │                 application/                 │
                  │              (api, future cli)               │
                  └──────────────┬────────────────┬──────────────┘
                                 │                │
             [Depends on Outbound│                │ [Depends on Core Use Cases
                  Port Impls]    │                │  and Inbound Ports]
                                 ▼                ▼
                  ┌────────────────────────┐    ┌────────────────────────┐
                  │    infrastructure/     │    │         domain         │
                  │        (sqlite)        │───▶│   (Core & Services)    │
                  └────────────────────────┘    └────────────────────────┘
```

1. **`domain` Crate**:
   > [!IMPORTANT]
   > `domain` must remain completely isolated. It **cannot** declare dependencies on `infrastructure/db` or `application/api` in its `Cargo.toml`. It is a pure library.
2. **`infrastructure/sqlite` Crate**:
   * Depends **only** on `domain` (to access domain entities and outbound repository traits) and database crates (e.g. `sqlx`).
   * It **cannot** depend on `application/api`.
3. **`application/api` Crate**:
   * Depends on **both** `domain` (for use cases) and `infrastructure/db` (for repositories).
   * It serves as the **Composition Root**, instantiating adapters and injecting them into domain services.

---

## 4. Formal BDD Requirements

### Requirement: Cargo Workspace Definition
The codebase MUST be initialized as a Cargo Workspace containing a root `Cargo.toml` file that specifies `domain`, `infrastructure/db`, and `application/api` as its workspace members.

#### Scenario: Workspace structure verification
- **WHEN** the project root Cargo.toml is parsed
- **THEN** it SHALL list `domain`, `infrastructure/db`, and `application/api` in its members array

### Requirement: Strict Unidirectional Layer Dependencies
The compilation dependencies between workspace members SHALL point strictly inward. The `domain` crate SHALL have no dependencies on other workspace crates. The `infrastructure/db` crate SHALL depend only on the `domain` crate. The `application/api` crate SHALL depend on both `domain` and `infrastructure/db`.

#### Scenario: Verify compiler separation
- **WHEN** compilation of the `domain` crate is performed
- **THEN** it MUST succeed without importing any DB or API libraries

### Requirement: Core Ports and Services Placement
All domain entities, inbound port use-case traits, outbound port repository traits, and application service orchestrators SHALL be co-located inside the `domain` library crate, organized into capability modules. Each capability module SHALL contain all artifacts belonging to that capability: its entity model(s), use case trait(s), outbound port trait(s), and service implementation(s).

#### Scenario: Retrieve ports and services
- **WHEN** another developer inspects the `domain` crate
- **THEN** it SHALL contain all entities, use-case traits, repository traits, and services
- **THEN** all artifacts for a single capability SHALL reside within the same capability module directory

### Requirement: Use-case-first module organization
All workspace crates SHALL organize their source code into capability modules rather than technical-layer directories. A capability module is a directory named after a business or cross-cutting concern (e.g. `health/`, `identity/`). Technical grouping directories (e.g. `models/`, `ports/`, `services/`, `extractors/`) SHALL NOT exist at any level within a crate.

The sole exception is `errors.rs` in the `domain` crate, which SHALL remain a flat file at the crate root as a cross-cutting concern shared by all capabilities.

#### Scenario: No technical-layer directories in domain
- **WHEN** the `domain` crate source tree is inspected
- **THEN** it SHALL NOT contain directories named `models`, `ports`, or `services`
- **THEN** it SHALL contain only capability module directories and the flat `errors.rs` file alongside `lib.rs`

#### Scenario: No technical-layer directories in infrastructure
- **WHEN** the `infrastructure/sqlite` crate source tree is inspected
- **THEN** it SHALL NOT contain flat repository files at the crate root (other than `lib.rs`)
- **THEN** it SHALL organize persistence adapters into capability module directories

#### Scenario: No technical-layer directories in application
- **WHEN** the `application/api` crate source tree is inspected
- **THEN** it SHALL NOT contain an `extractors/` directory or flat handler files under `http/`
- **THEN** it SHALL organize handlers and request-scoped extractors into capability module directories under `http/`

### Requirement: Action-oriented file naming within capability modules
Files within a capability module SHALL be named after the action or entity they represent, not after their technical role. Names such as `use_case.rs`, `service.rs`, `repository.rs`, and `inbound.rs` are forbidden. The use case trait, outbound port trait, and service implementation for a given use case SHALL be co-located in a single file named after the action (e.g. `check_health.rs`). Domain entity files SHALL be named after the entity (e.g. `system_health.rs`).

#### Scenario: Use case file is action-named
- **WHEN** a developer adds a new use case to a capability module
- **THEN** the file containing the use case trait, its outbound port(s), and service implementation SHALL be named after the action (e.g. `<verb>_<noun>.rs`)

#### Scenario: No role-named files inside a capability
- **WHEN** any capability module in any crate is inspected
- **THEN** it SHALL NOT contain files named `use_case.rs`, `service.rs`, `repository.rs`, `inbound.rs`, or `outbound.rs`

### Requirement: Capability naming symmetry across layers
A capability that spans multiple crates SHALL use the same capability name in each crate. The `domain` capability name SHALL be the authoritative reference; `infrastructure` and `application` layers SHALL mirror it.

#### Scenario: Health capability name is consistent across crates
- **WHEN** the source trees of `domain`, `infrastructure/sqlite`, and `application/api` are inspected
- **THEN** the health capability SHALL appear as `health/` in each crate that implements it

#### Scenario: Identity capability name is consistent across layers
- **WHEN** the source trees of `domain` and `application/api` are inspected
- **THEN** the identity capability SHALL appear as `identity/` in both crates

### Requirement: Async SQLite Persistence Adapter
The `infrastructure/sqlite` crate SHALL implement an async SQLite persistence adapter using SQLx, which executes all database operations asynchronously and automatically runs SQL schema migrations on application startup.

#### Scenario: Run startup migrations
- **WHEN** the `infrastructure/sqlite` adapter establishes a connection pool
- **THEN** it SHALL automatically run all migrations inside the migrations directory

### Requirement: API Executable and DI Composition Root
The `application/api` crate SHALL compile to an executable binary that uses Axum to expose a REST API health endpoint, and it SHALL serve as the dependency injection Composition Root that instantiates the SQLite database repository adapter and injects it into domain services.

#### Scenario: Serve health check endpoint
- **WHEN** a GET request is sent to `/health`
- **THEN** the system SHALL return an HTTP 200 OK status

---

## 5. Development Playbook: Adding a New Feature

To ensure consistent code quality and architectural integrity, all contributors must follow this playbook when adding new business capabilities. Code is organized into **capability modules** — directories named after the business concern, not the technical role.

As a running example, we illustrate adding a **"Create Product"** feature:

```
   1. Domain Capability         2. Infra Adapter              3. API Capability
 ┌──────────────────────┐     ┌──────────────────────┐     ┌──────────────────────┐
 │ domain/src/product/  │     │ sqlite/src/product/  │     │ http/product/        │
 │                      │     │                      │     │                      │
 │ product.rs           │     │ create_product.rs    │     │ create_product.rs    │
 │  └ Product entity    │     │  └ SqliteProductRepo ├────▶│  └ Axum handler      │
 │                      │     │    implements        │     │                      │
 │ create_product.rs    │     │    ProductRepository │     │ main.rs              │
 │  └ CreateProductUseCase    └──────────────────────┘     │  └ DI wiring         │
 │  └ ProductRepository ├──────────────────────────────────│                      │
 │  └ ProductService    │                                   └──────────────────────┘
 └──────────────────────┘
```

### Step 1: Create the capability module in `domain`
Create a directory `domain/src/product/`. Inside it, add:
- An **entity file** named after the model (e.g. `product.rs`) with the domain struct and its invariants.
- An **action file** named after the use case (e.g. `create_product.rs`) containing the inbound port trait, outbound port trait(s), and the service implementation.

```rust
// domain/src/product/product.rs
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: u64,
}

impl Product {
    pub fn new(name: String, price: u64) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::Validation("Product name cannot be empty".into()));
        }
        Ok(Self { id: uuid::Uuid::new_v4().to_string(), name, price })
    }
}
```

```rust
// domain/src/product/create_product.rs

// Outbound port (driven)
#[async_trait::async_trait]
pub trait ProductRepository: Send + Sync {
    async fn save(&self, product: &Product) -> Result<(), DomainError>;
}

// Inbound port (driving / use case)
#[async_trait::async_trait]
pub trait CreateProductUseCase: Send + Sync {
    async fn execute(&self, name: String, price: u64) -> Result<Product, DomainError>;
}

// Service implementation
pub struct ProductService {
    product_repo: Arc<dyn ProductRepository>,
}

impl ProductService {
    pub fn new(product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { product_repo }
    }
}

#[async_trait::async_trait]
impl CreateProductUseCase for ProductService {
    async fn execute(&self, name: String, price: u64) -> Result<Product, DomainError> {
        let product = Product::new(name, price)?;
        self.product_repo.save(&product).await?;
        Ok(product)
    }
}
```

Declare the module in `domain/src/lib.rs`:
```rust
pub mod product;
```

### Step 2: Implement the database adapter in `infrastructure/sqlite`
Create `infrastructure/sqlite/src/product/create_product.rs` with the SQLx implementation of `ProductRepository`.

```rust
// infrastructure/sqlite/src/product/create_product.rs
use domain::product::create_product::ProductRepository;
use domain::product::product::Product;

pub struct SqliteProductRepository {
    pool: Arc<sqlx::SqlitePool>,
}

#[async_trait::async_trait]
impl ProductRepository for SqliteProductRepository {
    async fn save(&self, product: &Product) -> Result<(), DomainError> {
        sqlx::query!(
            "INSERT INTO products (id, name, price) VALUES (?, ?, ?)",
            product.id, product.name, product.price
        )
        .execute(self.pool.as_ref())
        .await
        .map(|_| ())
        .map_err(|e| DomainError::Database(e.to_string()))
    }
}
```

Declare the module in `infrastructure/sqlite/src/lib.rs`:
```rust
pub mod product;
```

### Step 3: Implement the HTTP handler in `application/api`
Create `application/api/src/http/product/create_product.rs` with the Axum handler. The `AuthenticatedUser` extractor from `http::identity::authenticate` can be imported directly for any protected endpoint.

```rust
// application/api/src/http/product/create_product.rs
use axum::{extract::State, Json, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use domain::product::create_product::CreateProductUseCase;

pub async fn handle(
    State(use_case): State<Arc<dyn CreateProductUseCase>>,
    Json(payload): Json<CreateProductRequest>,
) -> impl IntoResponse {
    let product = use_case.execute(payload.name, payload.price).await?;
    (StatusCode::CREATED, Json(product))
}
```

### Step 4: Wire dependency injection in `main.rs`
Instantiate the repository, inject it into the service, and mount the route in the composition root.

```rust
// application/api/src/main.rs (Composition Root)
let product_repo = Arc::new(sqlite::product::SqliteProductRepository::new(pool.clone()));
let product_service = Arc::new(domain::product::ProductService::new(product_repo));

let app = Router::new()
    .route("/products", post(http::product::create_product::handle))
    .with_state(product_service as Arc<dyn CreateProductUseCase>);
```

---

## 6. Testing Standards

Test-driven design is heavily facilitated by this architecture.

### Unit Testing Core Logic (No DB Required)
Since the `domain` services rely strictly on ports, you can test all core logic without spinning up a real database. We use manual lightweight test doubles or `mockall` for repository traits.

```rust
// domain/src/product/create_product.rs (tests module)
struct InMemoryProductRepo {
    products: Mutex<Vec<Product>>,
}

#[async_trait::async_trait]
impl ProductRepository for InMemoryProductRepo {
    async fn save(&self, product: &Product) -> Result<(), DomainError> {
        self.products.lock().unwrap().push(product.clone());
        Ok(())
    }
}

#[tokio::test]
async fn test_create_product_success() {
    let repo = Arc::new(InMemoryProductRepo::new());
    let service = ProductService::new(repo.clone());
    
    let result = service.execute("Mechanical Keyboard".into(), 150).await;
    assert!(result.is_ok());
    assert_eq!(repo.products.lock().unwrap().len(), 1);
}
```

### Database Integration Testing
Place integration tests that verify database adapters inside the `infrastructure/sqlite` crate, utilizing an in-memory SQLite database (`sqlite::memory:`) to keep tests fast, isolated, and parallelized.
