# ADR 0001: Arquitetura hexagonal como estrutura inicial

- **Status:** Aceita
- **Data:** 2026-10-10

## Contexto

O template deve permitir que casos de uso sejam testados sem depender de HTTP, banco de dados ou serviços externos. Também deve ser simples adicionar novos microsserviços sem acoplar o domínio à infraestrutura.

## Decisão

Cada serviço Rust separa `domain`, `application`, `ports` e `adapters`. Portas definem contratos; adaptadores de entrada traduzem protocolos para casos de uso; adaptadores de saída implementam integrações externas.

## Consequências

- O domínio permanece independente de frameworks.
- Há mais arquivos para casos de uso pequenos.
- Adaptadores podem ser substituídos em testes ou na evolução do sistema.

