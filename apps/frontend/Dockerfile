# syntax=docker/dockerfile:1
# ============================================================
# Dockerfile multi-stage para frontend React/Vite
# Stack utilizada: Bun + React + Vite + Vitest + Playwright
#
# Uso:
#   docker build --target dev   -t app:dev   .
#   docker build --target test  -t app:test  .
#   docker build --target e2e   -t app:e2e   .
#   docker build --target prod  -t app:prod  .
# ============================================================

# ---------- Base ----------
FROM oven/bun:1 AS base
WORKDIR /app
COPY package.json bun.lock ./

# ---------- Dependencies (todas, incluindo dev) ----------
FROM base AS deps
RUN bun install --frozen-lockfile

# ---------- Development ----------
FROM deps AS dev
COPY . .
EXPOSE 5173
CMD ["bun", "run", "dev", "--", "--host", "0.0.0.0"]

# ---------- Test ----------
FROM deps AS test
COPY . .
CMD ["bun", "run", "test"]

# ---------- E2E (Playwright) ----------
FROM deps AS e2e
COPY . .
RUN bunx playwright install --with-deps chromium
EXPOSE 5173
CMD ["bun", "run", "test:e2e"]

# ---------- Build ----------
FROM deps AS build
COPY . .
RUN bun run build

# ---------- Production ----------
FROM nginx:1.27-alpine AS prod
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]