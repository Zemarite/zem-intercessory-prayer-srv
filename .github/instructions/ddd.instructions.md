# Rust Backend Generation with Domain-Driven Design (DDD)

This instruction set guides AI agents, such as GitHub Copilot, in generating consistent, production-quality Rust backends adhering to Domain-Driven Design (DDD) principles. It emphasizes architectural consistency, trigger accuracy, and best practices for scalable, maintainable code.

## Conventions and Best Practices

### DDD Principles
- **Bounded Contexts**: Organize code into bounded contexts representing distinct business domains. Use modules (e.g., `mod domain;`, `mod application;`) to encapsulate contexts.
- **Entities**: Represent domain objects with identity. Use structs with unique identifiers (e.g., UUIDs). Implement `PartialEq`, `Eq`, and `Hash` for equality and hashing.
- **Value Objects**: Immutable structs without identity. Implement `Clone`, `Copy` where appropriate, and derive `Debug`, `PartialEq`.
- **Repositories**: Interfaces for data access. Define traits (e.g., `trait UserRepository`) in the domain layer, implemented in infrastructure.
- **Services**: Application and domain services. Use structs or impl blocks for logic not belonging to entities/aggregates.
- **Events**: Use structs for domain events. Implement event sourcing where applicable.
- **CQRS**: Separate command and query responsibilities. Use separate handlers for writes (commands) and reads (queries).

### Rust-Specific Best Practices
- **Error Handling**: Use `Result<T, E>` with custom error types (e.g., `enum DomainError`). Leverage `thiserror` for error derivation.
- **Async/Await**: Use `tokio` for asynchronous operations. Prefer async traits for repositories and services.
- **Ownership and Borrowing**: Favor ownership over borrowing where possible. Use `Arc` for shared state in multi-threaded contexts.
- **Testing**: Write unit tests with `#[cfg(test)]` modules. Use `mockall` for mocking traits.
- **Dependencies**: Prefer crates like `serde` for serialization, `uuid` for IDs, `chrono` for dates.
- **Code Organization**: Structure as `src/domain/`, `src/application/`, `src/infrastructure/`, `src/interface/`. Use `lib.rs` for module declarations.
- **Naming**: Use snake_case for functions/variables, PascalCase for types. Prefix traits with `Trait` if needed for clarity.
- **Documentation**: Add `///` doc comments for public APIs. Use `#[derive(Debug)]` for debuggability.

### Architectural Consistency
- **Layer Separation**: Domain (core logic), Application (use cases), Infrastructure (external concerns), Interface (entry points).
- **Dependency Injection**: Use traits for interfaces, inject implementations (e.g., via constructors).
- **Immutability**: Prefer immutable data structures. Use `Rc` or `Arc` for shared references.
- **Performance**: Avoid unnecessary allocations. Use `Cow` for conditional ownership.

## General Instructions

### Trigger Accuracy
- Respond to prompts like "Generate a DDD-based Rust backend for [domain]" by scaffolding the full structure.
- For code modifications, ensure changes align with DDD layers (e.g., add domain logic to `domain/`).
- If a prompt lacks DDD context, ask for clarification or default to DDD patterns.

### Code Generation Guidelines
- Generate modular code: Start with `Cargo.toml` for dependencies, then `lib.rs` with modules.
- Include examples: Provide sample entities, repositories, and handlers.
- Handle Edge Cases: Account for validation, error propagation, and concurrency.
- Version Control: Suggest commits for each layer (e.g., "Add domain entities").

### Example Workflow
1. Parse user prompt for domain (e.g., "e-commerce").
2. Generate bounded context modules.
3. Implement core DDD elements (entities, aggregates, etc.).
4. Add application services and API endpoints.
5. Include tests and documentation.

This instruction ensures AI-generated code is production-ready, focusing on maintainability and scalability in Rust ecosystems.