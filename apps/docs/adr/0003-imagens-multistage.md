# ADR 0003: Imagens Docker multistage

- **Status:** Aceita
- **Data:** 2026-10-10

## Contexto

Compiladores, caches e dependências de desenvolvimento não devem ser distribuídos junto com as aplicações.

## Decisão

Os Dockerfiles usam uma etapa de build e uma etapa de runtime. A imagem Rust final contém apenas o binário, certificados e `wget` para healthcheck. A imagem frontend final contém somente os artefatos estáticos e o Nginx.

## Consequências

- Imagens finais menores e com menor superfície de ataque.
- O cache das dependências Rust e Node é preservado entre builds.
- Alterações em toolchains devem ser atualizadas explicitamente nos Dockerfiles.

