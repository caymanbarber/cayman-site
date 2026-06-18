# cayman.fyi

Personal portfolio and web app playground for Cayman Barber. Built to learn Rust, have fun, and let personality through.

## Architecture

```
backend/   — Rust (Axum) API server
frontend/  — Vite + Vanilla TypeScript
```

The backend serves API routes under `/api/*` and serves the frontend build as static files in production. During development, Vite proxies `/api` requests to the backend.

## Design System

- **Theme**: Dark only. No light mode.
- **Accent**: `#e05c3a` (rust-orange — intentional nod to Rust)
- **Fonts**: Space Grotesk (sans), JetBrains Mono (mono)
- **CSS**: Variables-first, all tokens in `frontend/src/styles/variables.css`
- **Vibe**: Serious but fun. Not corporate. Developer personality.

## Development

```bash
# Backend (from backend/)
cargo run                  # starts on :3000

# Frontend (from frontend/)
npm install                # first time only
npm run dev                # starts on :5173, proxies /api to :3000

# Docker (from root)
docker compose up --build  # backend :3000, frontend :8080
```

## API Routes

| Method | Path         | Description           |
|--------|--------------|-----------------------|
| GET    | /api/health  | Health check          |
| GET    | /api/hello   | Hello with ?name=     |

## Deployment Target

Raspberry Pi (ARM64) on local network. Docker images should build for `linux/arm64`. May eventually connect to AWS services (Lambda, S3, DynamoDB).

## Project Goals

1. Learn Rust by building real things
2. Fun portfolio that shows personality
3. Web apps and experiments connected together
4. Self-hosted on Raspberry Pi
5. AI handles most frontend work; human focuses on Rust

## Conventions

- **Rust**: Follow `cargo fmt` and `cargo clippy` defaults
- **TypeScript**: Strict mode, no `any`
- **CSS**: Use variables from `variables.css`, never hardcode colors/fonts
- **Git**: Short imperative commit messages
- **Files**: snake_case for Rust, camelCase for TypeScript
- **New routes**: Add file in `backend/src/routes/`, register in `mod.rs`
- **New pages/components**: Add in `frontend/src/`, import styles from `styles/`

## Testing

```bash
cargo test                # Rust tests (from backend/)
npx tsc --noEmit          # TypeScript type check (from frontend/)
```

CI runs both on every push to main and on PRs.
