# Rust Service Template

Template de referência para iniciar aplicações em Rust com arquitetura hexagonal, frontend moderno e gateway reverso em um único ambiente de desenvolvimento.

## Visão geral

Este repositório reúne uma estrutura prática para projetos full-stack com:

- `apps/example`: serviço backend em Rust com domínio, casos de uso, portas e adaptadores.
- `apps/frontend`: frontend em React + Vite.
- `nginx`: gateway reverso que entrega o frontend e encaminha requisições `/api` para o serviço backend.
- `compose.yaml`: ambiente local com rede interna, healthchecks e orquestração simples.
- `apps/docs/adr`: decisões arquiteturais documentadas em ADRs.
- Dockerfiles multistage para imagens finais leves e reproduzíveis.

## Pré-requisitos

- Docker Engine 24+
- Docker Compose v2
- Rust 1.88+ (opcional, para desenvolvimento local fora do Docker)
- Node.js 22+ (opcional, para desenvolvimento do frontend fora do Docker)

## Executando com Docker Compose

Na raiz do projeto:

```bash
docker compose up --build
```

Depois disso, o ambiente fica disponível em:

- Frontend: <http://localhost:8080>
- API: <http://localhost:8080/api>
- Healthcheck da API: <http://localhost:8080/api/health>

Para subir em segundo plano:

```bash
docker compose up --build -d
docker compose ps
docker compose logs -f app
```

Para encerrar:

```bash
docker compose down
```

Se quiser remover também volumes e redes criadas:

```bash
docker compose down -v
```

## Executando localmente

### Backend em Rust

```bash
cargo run --manifest-path apps/example/Cargo.toml
curl http://localhost:3000/health
```

### Frontend em React

Em outro terminal:

```bash
cd apps/frontend
npm install
npm run dev
```

A aplicação frontend costuma ficar disponível em <http://localhost:5173>.

## Estrutura do repositório

```text
.
├── apps/
│   ├── docs/
│   │   └── adr/
│   │       ├── 0001-arquitetura-hexagonal.md
│   │       ├── 0002-compose-e-nginx-como-borda.md
│   │       └── 0003-imagens-multistage.md
│   ├── example/
│   │   ├── src/
│   │   ├── Cargo.toml
│   │   ├── Cargo.lock
│   │   └── Dockerfile
│   └── frontend/
│       ├── src/
│       ├── package.json
│       ├── vite.config.ts
│       └── Dockerfile
├── nginx/
│   └── nginx.conf
├── compose.yaml
├── .env.example
├── CHANGELOG.md
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── LICENSE
├── README.md
└── .github/
```

## Fluxo de trabalho recomendado

1. Renomeie o pacote e os serviços de exemplo para refletirem o seu projeto.
2. Ajuste o backend para usar seu adaptador de persistência ou integração real.
3. Adicione casos de uso na camada de aplicação e exponha-os por portas de entrada.
4. Registre decisões importantes em `apps/docs/adr`.
5. Mantenha a configuração de container e healthcheck consistentes com o ambiente de produção.

## Comandos úteis

```bash
cargo fmt --manifest-path apps/example/Cargo.toml -- --check
cargo test --manifest-path apps/example/Cargo.toml
docker compose config
```

```bash
cd apps/frontend
npm run build
npm run test
```

## Observações

- O frontend e o backend podem ser desenvolvidos separadamente para maior velocidade de iteração.
- O ambiente Docker Compose é a forma mais simples de validar o comportamento completo do template.
- O repositório foi pensado como base para microsserviços e serviços com frontend, mas pode ser adaptado para projetos menores.
