# ADR 0002: Compose e Nginx como borda local

- **Status:** Aceita
- **Data:** 2026-10-10

## Contexto

O template precisa oferecer uma experiência local próxima de um ambiente distribuído: frontend, gateway e serviço devem ter responsabilidades e redes próprias.

## Decisão

O Docker Compose orquestra os serviços. O Nginx é o único serviço publicado no host, entrega o frontend e encaminha `/api` para os serviços internos. A rede `backend` é interna e o serviço Rust possui healthcheck.

## Consequências

- A topologia pode crescer adicionando serviços sem expor cada porta.
- O Nginx adiciona um componente operacional, mas centraliza roteamento e headers.
- A configuração de produção deve ser revisada para TLS, limites e observabilidade.

