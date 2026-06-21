# Mail Service

A lightweight Rust backend that handles email subscriber capture for the portfolio website. Exposes a REST API to save subscriber emails and send greeting/confirmation messages.

**Live URL:** https://mailapi.cursedshrine.com

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust (2021 edition) |
| Web Framework | Axum 0.7 |
| Async Runtime | Tokio |
| Database | (configurable via env) |
| Hosting | Raspberry Pi → Cloudflare Tunnel |

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/` | Health check — returns greeting |
| `POST` | `/save/subscriber` | Save a new email subscriber |

## Project Structure

```
Mail-Service/
├── src/
│   ├── main.rs              # Server bootstrap, routing, CORS
│   ├── controllers/
│   │   └── root.rs          # Route handlers
│   ├── services/
│   │   ├── greeting.rs      # Greeting logic
│   │   └── database_service.rs  # DB connection & ops
│   └── models/              # Data structs (Serde)
├── Cargo.toml
└── deploy.sh                # Build & restart helper
```

## Building & Running

```bash
cargo build --release
./target/release/mail-service
```

Server binds to `0.0.0.0:7070`.

## Deployment

```bash
sudo systemctl status mail-service
sudo systemctl restart mail-service
bash deploy.sh               # build + restart in one step
```

Port `7070` → Cloudflare Tunnel → `mailapi.cursedshrine.com`.

## CORS

CORS is configured to `*` (any origin) to allow calls from `gaurav.cursedshrine.com`.
