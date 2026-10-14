# syntax=docker/dockerfile:1
# ============================================================
# Dockerfile multi-stage para o serviço Rust
#
# Uso:
#   docker build --target dev  -t example-app:dev  .
#   docker build --target test -t example-app:test .
#   docker build --target prod -t example-app:prod .
# ============================================================

# ---------- Base ----------
FROM rust:1.88-bookworm AS base
WORKDIR /build
COPY Cargo.toml Cargo.lock* ./

# ---------- Dependencies (cache das dependências) ----------
FROM base AS deps
RUN mkdir src \
    && printf 'fn main() {}\n' > src/main.rs \
    && cargo build --release \
    && rm -rf src

# ---------- Development ----------
FROM deps AS dev
COPY src ./src
RUN touch src/main.rs
ENV APP_HOST=0.0.0.0 APP_PORT=3000 RUST_LOG=info
EXPOSE 3000
CMD ["cargo", "run", "--release"]

# ---------- Test ----------
FROM deps AS test
COPY src ./src
RUN touch src/main.rs
CMD ["cargo", "test"]

# ---------- Build ----------
FROM deps AS build
COPY src ./src
RUN touch src/main.rs \
    && cargo build --release

# ---------- Production ----------
FROM debian:bookworm-slim AS prod
RUN apt-get update \
    && apt-get install --no-install-recommends -y ca-certificates wget \
    && rm -rf /var/lib/apt/lists/*
RUN useradd --system --uid 10001 --create-home appuser
WORKDIR /app
COPY --from=build /build/target/release/example-app /usr/local/bin/example-app
USER appuser
ENV APP_HOST=0.0.0.0 APP_PORT=3000 RUST_LOG=info
EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/example-app"]



