# Mail Service

## Project Overview

Mail Service is a Rust-based HTTP microservice for managing email subscribers. It exposes a REST API that allows clients to register subscribers into a MongoDB database. The service is designed as a notification/mailing backend — clients POST subscriber data (name + email) along with a `clientId` header to tag which client they belong to.

The project is early-stage (v0.1.0) and currently implements two endpoints: a health/greeting check and a subscriber registration endpoint.

## Tech Stack / Crates

| Crate | Version | Purpose |
|---|---|---|
| `axum` | 0.7 | Async HTTP web framework (routing, extractors, state) |
| `tokio` | 1 (full) | Async runtime |
| `tower` | 0.5 | Middleware/service utilities |
| `tower-http` | 0.6 | HTTP middleware — CORS layer used here |
| `hyper` | 1.8 | HTTP primitives (used for `hyper::Method` in CORS config) |
| `serde` | 1.0 | Serialization/deserialization with `derive` feature |
| `mongodb` | 3.4 | Official async MongoDB driver |
| `dotenv` | 0.15 | Load environment variables from `.env` file |

Rust edition: **2024**

## Build and Run

### Prerequisites

- Rust toolchain (stable, edition 2024 support requires 1.85+)
- MongoDB instance (local or remote)

### Environment Setup

Create a `.env` file in the project root (it is gitignored — never commit it):

```
MONGODB_URI=mongodb://localhost:27017/
```

The `.env.example` file is the safe-to-commit template (add one if it doesn't exist yet).

### Commands

```bash
# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run (debug, loads .env automatically)
cargo run

# Run tests
cargo test
```

The server binds to `0.0.0.0:7070` on startup. No port is configurable via env at the moment — it is hardcoded in `main.rs`.

## API Endpoints

| Method | Path | Description |
|---|---|---|
| `GET` | `/` | Health check — returns "Mailing Service Activated..." |
| `POST` | `/save/subscriber` | Register a new subscriber |

### POST `/save/subscriber`

**Required header:**
```
clientId: <string>
```

**Request body (JSON):**
```json
{
  "name": "Jane Doe",
  "email": "jane@example.com"
}
```

**Responses:**
- `201 Created` — subscriber saved successfully
- `400 Bad Request` — `clientId` header is missing
- `500 Internal Server Error` — MongoDB write failed

CORS is open (`*`) for all origins, allowing `GET` and `POST` methods and any headers.

## Key Files and Their Purpose

```
Mail-Service/
├── Cargo.toml                          # Project manifest and dependencies
├── .env                                # Local env vars (gitignored)
├── src/
│   ├── main.rs                         # Entry point: builds AppState, wires router, starts server
│   ├── controllers/
│   │   ├── mod.rs                      # Re-exports controller modules
│   │   └── root.rs                     # HTTP handler functions (say_hello, save_subscriber)
│   ├── models/
│   │   ├── mod.rs                      # Re-exports model modules
│   │   └── subscriber.rs               # Subscriber struct (name, email, client_id)
│   └── services/
│       ├── mod.rs                      # Re-exports service modules
│       ├── greeting.rs                 # GreetingService — returns startup message string
│       └── database_service.rs         # DatabaseService — wraps MongoDB collection, inserts subscribers
```

### AppState

`AppState` in `main.rs` is the shared state injected into every handler via axum's `State` extractor. It holds:
- `greeting_service: GreetingService`
- `database_service: DatabaseService`

Both are `Clone` so axum can cheaply share them across requests.

### Database

- MongoDB database name: `notification_service`
- Collection: `subscribers`
- Connection URI comes from the `MONGODB_URI` environment variable (required — panics if missing)

## Important Notes for Developers

- **Port is hardcoded** to `7070`. If you need it configurable, read it from an env var in `main.rs`.
- **CORS is fully open** (`allow_origin(Any)`). Before deploying to production, restrict origins appropriately.
- **No authentication** is implemented yet. The `clientId` header is trusted as-is from the caller.
- **No deduplication** — the same email can be inserted multiple times. Add a unique index on `email` + `client_id` in MongoDB if needed.
- **No logging framework** is set up — the code uses `println!` for debug output. Consider adding `tracing`/`tracing-subscriber` for structured logging.
- The `.env` file is gitignored. Always provide `.env.example` with placeholder values so other developers know what variables are required.
