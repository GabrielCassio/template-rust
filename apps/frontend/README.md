# Frontend

Frontend React com Vite, TypeScript, Vitest, Playwright, ESLint e Prettier.

To install dependencies:

```bash
bun install
```

To run the development server:

```bash
bun run dev
```

To validate the projeto:

```bash
bun run typecheck
bun run lint
bun run format:check
bun run test
bun run test:e2e
```

O teste E2E inicia automaticamente o servidor Vite. Na primeira execução, instale o navegador do Playwright:

```bash
bunx playwright install chromium
```
