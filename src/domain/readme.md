# Domain Layer

The **Domain** project contains the core business logic of the application.

## Responsibilities

- **Entities**
  - that are related and should change together should be grouped into an Aggregate
  - should leverage encapsulation and should minimize public setters.
  - can leverage Domain Events to communicate changes to other parts of the system.
  - can define Specifications that can be used to query for them.
  
- **Value Objects** (immutable, equality by attributes) – src/domain/value_objects/
  - should be immutable and should implement equality based on their properties.
- **Repository Traits** (in src/domain/repositories/)
- **Domain Events**
- **Business rules and invariants**  

## Dependencies

- The Domain layer **must not depend on any other project**.
- It contains pure business logic only.

## Notes

This layer remains stable even if infrastructure, UI, or application code changes. It represents the heart of the system.
