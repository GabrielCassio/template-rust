# Rust Service Template

Template agnóstico para iniciar projetos completos em Rust com uma organização voltada a microsserviços e arquitetura hexagonal.

## O que está incluído

- `app`: serviço Rust de exemplo com domínio, portas e adaptadores separados.
- `frontend`: frontend estático mínimo, compilado com Vite.
- `nginx`: gateway reverso que entrega o frontend e encaminha `/api` para o serviço Rust.
- `compose.yaml`: ambiente local com rede interna, healthcheck e volumes de desenvolvimento.
- `docs/adr`: decisões arquiteturais registradas como ADRs.
- Dockerfiles multistage para produzir imagens finais pequenas e sem ferramentas de build.

## Pré-requisitos

- Docker Engine 24+ e Docker Compose v2.
- Rust 1.88+ apenas para desenvolvimento fora do Docker (opcional).
- Node.js 22+ apenas para desenvolvimento do frontend fora do Docker (opcional).

## Executando com Docker Compose

```bash
docker compose up --build
```

Abra <http://localhost:8080>. A API também pode ser verificada diretamente em <http://localhost:8080/api/health>.

Para executar em segundo plano:

```bash
docker compose up --build -d
docker compose ps
docker compose logs -f app
```

Para desligar e remover os containers:

```bash
docker compose down
```

## Executando localmente

```bash
cargo run --manifest-path app/Cargo.toml
curl http://localhost:3000/health
```

Em outro terminal:

```bash
cd frontend
npm install
npm run dev
```

## Estrutura

```text
├── apps/
│    ├── example/
│    │   ├── src/
│    │   │   ├── adapters/inbound/http
│    │   │   ├── adapters/outbound
│    │   │   ├── application
│    │   │   ├── domain
│    │   │   └── ports
│    │   └── Dockerfile
│    ├── frontend/
│    │   ├── src/
│    │   └── Dockerfile
│    ├── nginx/
│    │   └── nginx.conf
│    └── docs/adr/
└── compose.yaml
```

## Próximos passos

1. Renomeie o crate e o serviço em `app/Cargo.toml` e `compose.yaml`.
2. Substitua o adaptador de memória por um adaptador de persistência.
3. Adicione novos casos de uso em `application` e exponha-os por portas de entrada.
4. Registre decisões relevantes em `docs/adr`.
5. Crie novos serviços no Compose mantendo uma imagem multistage e um healthcheck.

## Comandos úteis

```bash
cargo fmt --manifest-path app/Cargo.toml -- --check
cargo test --manifest-path app/Cargo.toml
docker compose config
```
