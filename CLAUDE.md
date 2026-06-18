# cayman-site

Personal/professional site for Cayman Barber, deployed at [cayman.fyi](https://cayman.fyi) via GitHub Pages.

## Project phases

- **Phase 1 (now):** Static `index.html` placeholder. No build tools, no JS, no frameworks. Pure HTML + embedded CSS.
- **Phase 2:** Rust backend (Axum), API endpoints.
- **Phase 3:** Full portfolio — projects, blog, contact form.
- **Phase 4:** Custom domain (`cayman.fyi`) fully wired, possible WASM experiments.

## Design decisions

- **Accent color `#e05c3a`** is rust-orange intentionally — nod to the Rust language. Carry this forward.
- **Font pair:** Space Grotesk (headings) + JetBrains Mono (mono/code elements). Both chosen to read as serious-but-technical. Do not swap these without good reason.
- **All colors via CSS custom properties** — never hardcode hex values in component styles. The variable system in `:root` is the design token layer.
- **Dark palette** is hardcoded (not `prefers-color-scheme`). The dark-first aesthetic is intentional for the startup/dev crowd.

## Conventions

- No inline styles.
- Class names are BEM-ish but not strict BEM.
- Style block order: reset → variables → base → components → utilities → responsive.
- `.dog-placeholder` should eventually become an `<img>` tag pointing to a real photo. The CSS class and surrounding markup stay — just swap the inner content.
- The `// status: compiling...` badge with the pulsing dot should transition to something real (e.g., a build status indicator) once the backend exists.

## Key files

| File | Purpose |
|------|---------|
| `index.html` | Entire site for Phase 1 |
| `CLAUDE.md` | This file — project context for future sessions |
| `.gitignore` | Web + Rust ignores |
