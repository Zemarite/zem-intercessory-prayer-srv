# Infrastructure Layer

This directory contains all **infrastructure-specific** code — that is, everything that connects the core business logic (domain + application layers) to the external world.

The infrastructure layer is **the outermost layer** in clean/hexagonal/onion architecture and should contain:

- Database access (ORMs, raw SQL drivers, connection pools)
- External service clients (payment gateways, email, SMS, OAuth providers…)
- Messaging (queues, pub/sub, event buses)
- File/blob storage
- Any framework/library-specific adapters & concrete implementations

**Important principle**:  
No domain or application layer code is allowed to depend on this layer.  
Dependencies point **inward** — infrastructure depends on domain/application, never the other way around.

## Directory Structure

```text
src/infrastructure/
├── external_services/          # Clients & adapters for third-party HTTP APIs & services
│   ├── email/                  # Email sending (SendGrid, AWS SES, Postmark, Mailgun…)
│   ├── oauth_providers/        # Google, GitHub, Apple, Auth0, Okta, etc.
│   ├── payment_gateway/        # Stripe, PayPal, Adyen, Square, Mollie…
│   ├── sms/                    # Twilio, Vonage, MessageBird, AWS SNS…
│   └── third_party_apis/       # Other external APIs not fitting above categories
│
├── messaging/                  # All message broker / eventing related code
│   ├── consumers/              # Message / event handlers & workers
│   ├── event_bus/              # Pub/sub abstractions & concrete implementations
│   └── publishers/             # Code that produces messages/events
│
├── persistence/                # Everything related to data storage & retrieval
    ├── databases/   
    │   └── <specific_database>/            # Connection pools, migrations, query builders, raw drivers
    └── repositories/
        └── <specific_database>/           # Concrete implementations of domain repositories
│
├── storage_systems/            # File and object storage integrations
│   ├── blob_storage/           # S3, GCS, Azure Blob, MinIO, Backblaze…
│   └── file_storage/           # Local filesystem, SFTP, or legacy file servers
│
└── mod.rs                      # Public module exports
```
