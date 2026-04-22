## Workspace Structure

```
crates/
├── api/                # HTTP layer — routes, handlers, extractors
│   └── src/
│       └── lib.rs
│
├── app/                # Application layer — services & ports
│   └── src/
│       ├── lib.rs
│       └── features/
│           ├── auth/
│           │   ├── mod.rs
│           │   ├── port.rs
│           │   └── service.rs
│           └── user/
│               ├── mod.rs
│               ├── port.rs
│               └── service.rs
│
├── domain/             # Domain layer — entities, enums, value objects
│   └── src/
│       ├── lib.rs
│       └── features/
│           ├── auth/
│           │   ├── mod.rs
│           │   ├── entities/
│           │   └── enums/
│           └── user/
│               ├── mod.rs
│               ├── entities/
│               │   ├── mod.rs
│               │   └── user.rs
│               └── enums/
│
└── infrastructure/     # Infrastructure layer — repos, DB, external services
    └── src/
        ├── lib.rs
        └── features/
            └── user/
                ├── mod.rs
                └── repo.rs

migrations/             # SQL migration files
```
