# Session Handoff — cayman.fyi Setup (2026-06-19)

Give this file to Claude Code locally so it has full context from the remote session.

---

## What Was Done

Scaffolded the full monorepo boilerplate on branch `claude/init-web-app-m37629`. The live GitHub Pages site on `main` (index.html + CNAME + bingo.PNG) is untouched.

### Files Created
```
backend/                    — Rust (Axum) API server
  Cargo.toml                — axum, tokio, serde, tower-http, tracing
  Dockerfile                — multi-stage build, release-optimized, Pi-ready
  src/main.rs               — server setup with CORS, tracing, static file serving
  src/config.rs             — port/static_dir from env vars (PORT, STATIC_DIR)
  src/errors.rs             — AppError type (scaffolding for future routes)
  src/routes/mod.rs         — route registration
  src/routes/health.rs      — GET /api/health → { status: "ok" }
  src/routes/hello.rs       — GET /api/hello?name=X → { message: "Hello, X!" }

frontend/                   — Vite + Vanilla TypeScript
  package.json              — vite + typescript devDependencies
  tsconfig.json             — strict mode, no any
  vite.config.ts            — dev proxy /api → localhost:3000
  index.html                — Vite entry, same look as current site
  Dockerfile                — multi-stage: node build → caddy serve
  Caddyfile                 — reverse proxy /api/* to backend, serve static
  nginx.conf                — DELETED, replaced by Caddy
  src/main.ts               — entry point, checks backend health
  src/api.ts                — typed fetch wrapper for backend calls
  src/styles/variables.css  — design tokens (colors, fonts, spacing)
  src/styles/reset.css      — CSS reset
  src/styles/main.css       — base styles ported from original site
  src/assets/.gitkeep       — placeholder for images/fonts

docker-compose.yml          — backend + caddy services, single entry on :8080
.github/workflows/ci.yml    — cargo fmt/clippy/test + tsc --noEmit
.claude/settings.json       — pre-approved commands for AI workflow
.claude/commands/dev.md     — custom /dev slash command
CLAUDE.md                   — project overview for AI context
.gitignore                  — updated: tracks CLAUDE.md, scoped ignores
```

### Verified
- `cargo check` + `cargo clippy -- -D warnings` — pass clean
- `npx tsc --noEmit` — pass clean
- All pushed to `claude/init-web-app-m37629`

---

## Decisions Made

| Decision | Choice | Why |
|----------|--------|-----|
| Rust framework | Axum | Best for learning, Tokio ecosystem, good docs |
| Frontend | Vite + Vanilla TS | No framework overhead, AI handles most frontend |
| Reverse proxy | Caddy (not nginx) | Simpler config, auto-TLS, single binary |
| Docker | Yes, with Compose | Local dev + Pi deployment |
| CI | GitHub Actions | cargo fmt/clippy/test + tsc on PRs and main |
| Branch strategy | Feature branch, main untouched | Live GH Pages site stays up |

---

## Topics Discussed (Not Yet Implemented)

### HTTPS Strategy
Caddy handles HTTPS entirely — Axum stays HTTP internally. Progression:
1. **Now**: `:8080` (no TLS, local dev)
2. **Pi on LAN**: change Caddyfile line 1 to `https://pi.local`, add `tls internal` — self-signed, browsers warn but encrypted
3. **Public**: change to `cayman.fyi` — Caddy auto-provisions Let's Encrypt certs, needs ports 80+443 forwarded

**Tailscale vs Cloudflare Tunnel** (can use both):
- **Tailscale** — private mesh VPN for dev access. Install on Pi + laptop, get a `.ts.net` hostname. No ports opened. Great for SSH and admin access during development.
- **Cloudflare Tunnel** — public access for the portfolio. Install `cloudflared` on Pi, point domain through Cloudflare. No ports opened, free HTTPS/DDoS/CDN. Use when ready to go public.
- **Recommended**: Tailscale for dev/admin, Cloudflare Tunnel for public site. Both at same time.

### Server Security (Admin Endpoints)
When adding endpoints that control the Pi:
- **Auth**: API key in `Authorization: Bearer <key>` header, checked by an Axum extractor. Key stored in `.env` (gitignored).
- **Authorization**: Never run raw shell commands from user input. Define a fixed set of allowed actions in Rust (`match action { "restart" => ..., "deploy" => ... }`).
- **Network**: Public endpoints through Cloudflare, admin endpoints behind Tailscale only. UFW firewall on Pi.
- **Rate limiting**: tower middleware on `/api/admin/*`.

### IoT Device Integration
- **Devices → Server**: HTTP POST to `/api/devices/data` with device-specific API keys. Start with this. MQTT later if many constrained devices.
- **Server → Browser**: SSE (Server-Sent Events) for live dashboards. Axum has built-in `Sse` response type, browser uses native `EventSource` API. Uses `tokio::sync::broadcast` channel to fan out events.
- **No WebSockets needed** unless browser needs to send commands back to devices.
- **Device networking**: Devices on same LAN hit Pi's local IP directly. Remote devices use Tailscale.

Flow: `IoT Device → POST /api/devices/data → Axum stores + broadcasts → SSE stream → Browser updates live`

---

## Next Steps (Suggested Order)

1. **Run it locally** — `cargo run` in backend/, `npm run dev` in frontend/, verify the health check works
2. **Docker test** — `docker compose up --build`, verify Caddy proxies correctly on :8080
3. **Add auth extractor** — `backend/src/auth.rs` with API key checking
4. **Add SSE endpoint** — `backend/src/routes/events.rs` with broadcast channel
5. **Add device ingestion** — `backend/src/routes/devices.rs` for IoT POST
6. **Deploy to Pi** — Docker Compose, self-signed TLS first
7. **Go public** — Cloudflare Tunnel + real domain

---

## How to Add New Features

**New API route:**
1. Create `backend/src/routes/your_route.rs`
2. Add `mod your_route;` and `.merge(your_route::routes())` in `backend/src/routes/mod.rs`

**New frontend page/component:**
1. Add files in `frontend/src/`
2. Import styles from `styles/`, use CSS variables from `variables.css`

**New CSS tokens:**
1. Add to `frontend/src/styles/variables.css`
2. Reference with `var(--your-token)` everywhere

---

## Branch Info
- **Working branch**: `claude/init-web-app-m37629`
- **Live site**: `main` (GitHub Pages, index.html + CNAME)
- **Remote**: `origin` → `github.com/caymanbarber/cayman-site`
