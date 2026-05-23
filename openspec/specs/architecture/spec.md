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
├── Cargo.toml                  # Workspace root (registers all crates)
├── domain/                     # Crate (Library): Business logic, Ports, and Services
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── errors.rs           # Core domain error types
│       ├── models/             # Pure domain entities & value objects
│       ├── ports/              # Trait definitions for use cases & repositories
│       │   ├── inbound.rs      # Driving ports (Use Cases)
│       │   └── outbound.rs     # Driven ports (Repositories)
│       └── services/           # Concrete services implementing use cases
├── infrastructure/             # Outbound Adapters Group
│   └── db/                     # Crate (Library): SQLite database repository (SQLx)
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs          # SQLx pools, configuration, and migrations
│           └── migrations/     # SQL migration scripts (.sql)
└── application/                # Inbound Delivery & Executables Group
    └── api/                    # Crate (Binary): HTTP Web Server (Axum)
        ├── Cargo.toml
        └── src/
            ├── main.rs         # HTTP Composition Root & Dependency Injection
            └── http/           # HTTP Routing, handlers, and request DTOs
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
                  │          (db)          │───▶│   (Core & Services)    │
                  └────────────────────────┘    └────────────────────────┘
```

1. **`domain` Crate**:
   > [!IMPORTANT]
   > `domain` must remain completely isolated. It **cannot** declare dependencies on `infrastructure/db` or `application/api` in its `Cargo.toml`. It is a pure library.
2. **`infrastructure/db` Crate**:
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
All domain entities, inbound port use-case traits, outbound port repository traits, and application service orchestrators SHALL be co-located inside the `domain` library crate.

#### Scenario: Retrieve ports and services
- **WHEN** another developer inspects the `domain` crate
- **THEN** it SHALL contain all entities, use-case traits, repository traits, and services

### Requirement: Async SQLite Persistence Adapter
The `infrastructure/db` crate SHALL implement an async SQLite persistence adapter using SQLx, which executes all database operations asynchronously and automatically runs SQL schema migrations on application startup.

#### Scenario: Run startup migrations
- **WHEN** the `infrastructure/db` adapter establishes a connection pool
- **THEN** it SHALL automatically run all migrations inside the migrations directory

### Requirement: API Executable and DI Composition Root
The `application/api` crate SHALL compile to an executable binary that uses Axum to expose a REST API health endpoint, and it SHALL serve as the dependency injection Composition Root that instantiates the SQLite database repository adapter and injects it into domain services.

#### Scenario: Serve health check endpoint
- **WHEN** a GET request is sent to `/health`
- **THEN** the system SHALL return an HTTP 200 OK status

---

## 5. Development Playbook: Adding a New Feature

To ensure consistent code quality and architectural integrity, all contributors must follow this 5-step playbook when adding new business capabilities. 

As a running example, we illustrate adding a **"Create Product"** feature:

```
   1. Domain Models         2. Domain Ports             3. App Service             4. DB Adapter             5. API Routing
 ┌──────────────────┐     ┌──────────────────┐        ┌──────────────────┐       ┌──────────────────┐      ┌──────────────────┐
 │ Define Product   │     │ Outbound: Repo   │        │ Implement Use    │       │ Implement SQL    │      │ Axum Handler &   │
 │ struct & errors  ├────▶│ Inbound: UseCase ├───────▶│ Case Service     ├──────▶│ query in infra/db├─────▶│ state-injection  │
 │ in `domain`      │     │ traits in `ports`│        │ using outbound   │       │ (SQLx)           │      │ in `application` │
 └──────────────────┘     └──────────────────┘        └──────────────────┘       └──────────────────┘      └──────────────────┘
```

### Step 1: Define Domain Models & Errors
Define the raw business struct and rules in `domain/src/models/` and any feature-specific errors in `domain/src/errors.rs`.
```rust
// domain/src/models/product.rs
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
        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            price,
        })
    }
}
```

### Step 2: Define Ports (Interfaces)
Establish the interface requirements in the `domain/src/ports/` directory.

```rust
// domain/src/ports/outbound.rs (Driven Port)
#[async_trait::async_trait]
pub trait ProductRepository: Send + Sync {
    async fn save(&self, product: &Product) -> Result<(), DomainError>;
}

// domain/src/ports/inbound.rs (Driving Port / Use Case)
#[async_trait::async_trait]
pub trait CreateProductUseCase: Send + Sync {
    async fn execute(&self, name: String, price: u64) -> Result<Product, DomainError>;
}
```

### Step 3: Implement the Service
Write the application service coordinating the ports in `domain/src/services/`. Note that it accepts the repo as a dynamic trait reference wrapped in an `Arc`.

```rust
// domain/src/services/product_service.rs
use std::sync::Arc;
use crate::models::product::Product;
use crate::ports::outbound::ProductRepository;
use crate::ports::inbound::CreateProductUseCase;

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

### Step 4: Implement Database Persistence (Infrastructure)
Implement the outbound port inside the SQLite `infrastructure/db` crate.

```rust
// infrastructure/db/src/product_repository.rs
use domain::models::product::Product;
use domain::ports::outbound::ProductRepository;

pub struct SqliteProductRepository {
    pool: sqlx::SqlitePool,
}

#[async_trait::async_trait]
impl ProductRepository for SqliteProductRepository {
    async fn save(&self, product: &Product) -> Result<(), DomainError> {
        sqlx::query!(
            "INSERT INTO products (id, name, price) VALUES (?, ?, ?)",
            product.id, product.name, product.price
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::Database(e.to_string()))?;
        Ok(())
    }
}
```

### Step 5: Wire Dependency Injection & HTTP routing (Application)
Expose the capability as an HTTP route in `application/api` by extracting the state container.

```rust
// application/api/src/http/product.rs
async fn create_product(
    State(use_case): State<Arc<dyn CreateProductUseCase>>,
    Json(payload): Json<CreateProductRequest>,
) -> impl IntoResponse {
    let product = use_case.execute(payload.name, payload.price).await?;
    (StatusCode::CREATED, Json(product))
}

// application/api/src/main.rs (Composition Root)
let pool = db::establish_connection("sqlite:loggia.db").await?;
let product_repo = Arc::new(db::SqliteProductRepository::new(pool));
let product_service = Arc::new(domain::services::ProductService::new(product_repo));

let app = Router::new()
    .route("/products", post(create_product))
    .with_state(product_service as Arc<dyn CreateProductUseCase>);
```

---

## 6. Testing Standards

Test-driven design is heavily facilitated by this architecture.

### Unit Testing Core Logic (No DB Required)
Since the `domain` services rely strictly on ports, you can test all core logic without spinning up a real database. We use manual lightweight test doubles or `mockall` for repository traits.

```rust
// domain/src/services/product_service_tests.rs
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
Place integration tests that verify database adapters inside the `infrastructure/db` crate, utilizing an in-memory SQLite database (`sqlite::memory:`) to keep tests fast, isolated, and parallelized.
