# Interface Layer

The **interface layer** (sometimes called "delivery layer", "adapters" or "presentation layer") is responsible for:

- Receiving incoming requests from the outside world
- Parsing and validating input data
- Calling appropriate application/domain services
- Formatting responses (JSON, HTML, etc.)
- Handling cross-cutting concerns (auth, CORS, logging, rate limiting)

This layer **does not contain business logic** — it only translates between external protocols (HTTP, gRPC, CLI, WebSocket…) and the internal application/domain layer.

## Directory Structure

``` folder
src/interface/
├── authentication/     # Authentication & authorization mechanisms
│ ├── oauth2/           # OAuth2 flows (Google, GitHub, etc.)
│ ├── jwt/              # JWT creation, validation, refresh
│ └── cors/             # CORS configuration & middleware
│
├── http/               # HTTP-specific implementation (most common interface)
│ ├── handlers/         # Request handlers / controllers / endpoints logic
│ │ ├── auth.rs
│ │ ├── users.rs
│ │ ├── products.rs
│ │ └── mod.rs
│ │
│ └── routes/           # Route definitions & grouping
│ ├── auth_routes.rs
│ ├── api_v1.rs
│ ├── public.rs
│ └── mod.rs
│
└── mod.rs
```
