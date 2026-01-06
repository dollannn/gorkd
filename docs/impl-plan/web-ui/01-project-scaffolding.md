# Phase 1: Project Scaffolding

**Status**: Planned
**Duration**: ~0.5 days
**Depends on**: None

## Overview

Initialize SvelteKit project with Bun, TypeScript, and Tailwind CSS. Configure tooling (Prettier, ESLint, Vitest) and verify everything builds.

## Tasks

| Task | Description |
|------|-------------|
| Create SvelteKit project | `bunx sv create web` with TypeScript |
| Install Tailwind CSS v4 | Via SvelteKit add-on |
| Configure Prettier | Match project conventions (tabs, single quotes, no semis) |
| Configure ESLint | Svelte + TypeScript rules |
| Setup Vitest | Unit test runner |
| Add Lucide icons | `bun add lucide-svelte` |
| Create .env file | PUBLIC_API_URL variable |
| Verify build | `bun run build` succeeds |
| Update root README | Add web/ commands |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `web/` | **New folder** - SvelteKit project root |
| `web/package.json` | **New file** - Dependencies and scripts |
| `web/svelte.config.js` | **New file** - SvelteKit config |
| `web/vite.config.ts` | **New file** - Vite config with Vitest |
| `web/tsconfig.json` | **New file** - TypeScript config |
| `web/tailwind.config.ts` | **New file** - Tailwind config |
| `web/src/app.css` | **New file** - Global styles, Tailwind imports |
| `web/src/app.html` | **New file** - HTML template |
| `web/src/app.d.ts` | **New file** - App-level types |
| `web/.prettierrc` | **New file** - Formatter config |
| `web/.eslintrc.cjs` | **New file** - Linter config |
| `web/.env` | **New file** - Environment variables |
| `web/.env.example` | **New file** - Env template |
| `README.md` | Add web/ section to quickstart |

## Key Considerations

- Use Svelte 5 (runes mode) - ensure `sv create` uses latest
- Tailwind v4 uses CSS-based config, not JS
- Match Prettier config to root project (.prettierrc in AGENTS.md)
- Set `adapter-auto` for now, can switch to `adapter-node` later

## Prettier Config

```json
{
  "useTabs": true,
  "singleQuote": true,
  "semi": false,
  "trailingComma": "es5",
  "plugins": ["prettier-plugin-svelte"],
  "overrides": [{ "files": "*.svelte", "options": { "parser": "svelte" } }]
}
```

## Deliverables

- [ ] `bun install` succeeds
- [ ] `bun dev` starts dev server on port 5173
- [ ] `bun run build` succeeds
- [ ] `bun run check` passes (svelte-check)
- [ ] `bun run lint` passes
- [ ] `bun test` runs (even if no tests yet)
- [ ] Tailwind classes work in a test component
