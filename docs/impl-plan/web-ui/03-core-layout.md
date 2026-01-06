# Phase 3: Core Layout

**Status**: Planned
**Duration**: ~1 day
**Depends on**: Phase 1 (Project Scaffolding)

## Overview

Build the app shell, design tokens, and base UI components. Establish visual language that carries through all screens.

## Tasks

| Task | Description |
|------|-------------|
| Define design tokens | Colors, spacing, typography in CSS variables |
| Create app layout | Header, main content area, footer |
| Build Logo component | Simple text logo or icon |
| Build Button component | Primary, secondary, ghost variants |
| Build Input component | Text input with label, error state |
| Build Card component | Container with optional header |
| Build Spinner component | Loading indicator |
| Build Icon wrapper | Lucide icon with size props |
| Setup dark mode | CSS variables + toggle mechanism |
| Create +layout.svelte | Root layout with shell |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `web/src/app.css` | Design tokens, dark mode variables |
| `web/src/routes/+layout.svelte` | **New file** - App shell |
| `web/src/lib/components/Logo.svelte` | **New file** - Brand mark |
| `web/src/lib/components/Button.svelte` | **New file** - Button variants |
| `web/src/lib/components/Input.svelte` | **New file** - Form input |
| `web/src/lib/components/Card.svelte` | **New file** - Content container |
| `web/src/lib/components/Spinner.svelte` | **New file** - Loading state |
| `web/src/lib/components/Icon.svelte` | **New file** - Icon wrapper |
| `web/src/lib/components/ThemeToggle.svelte` | **New file** - Dark mode switch |
| `web/src/lib/components/index.ts` | **New file** - Component exports |
| `web/src/lib/stores/theme.svelte.ts` | **New file** - Theme state (runes) |

## Design Tokens

```css
:root {
  /* Colors - Slate palette */
  --color-bg: theme(colors.slate.50);
  --color-bg-subtle: theme(colors.slate.100);
  --color-text: theme(colors.slate.900);
  --color-text-muted: theme(colors.slate.500);
  --color-border: theme(colors.slate.200);
  --color-accent: theme(colors.blue.600);
  --color-accent-hover: theme(colors.blue.700);
  
  /* Spacing */
  --space-1: 0.25rem;
  --space-2: 0.5rem;
  --space-3: 0.75rem;
  --space-4: 1rem;
  /* ... */
  
  /* Typography */
  --font-sans: system-ui, -apple-system, sans-serif;
  --font-mono: ui-monospace, monospace;
}

.dark {
  --color-bg: theme(colors.slate.900);
  --color-bg-subtle: theme(colors.slate.800);
  --color-text: theme(colors.slate.50);
  /* ... */
}
```

## Component Props Pattern

```typescript
// Consistent prop interface pattern
interface ButtonProps {
  variant?: 'primary' | 'secondary' | 'ghost'
  size?: 'sm' | 'md' | 'lg'
  disabled?: boolean
  loading?: boolean
}
```

## Key Considerations

- Use CSS custom properties for theming (works with Tailwind)
- Dark mode via class on `<html>` element
- Persist theme preference to localStorage
- Respect `prefers-color-scheme` on first visit
- Components use Svelte 5 `$props()` rune
- No component library dependencies

## App Shell Structure

```
+------------------------------------------+
|  [Logo]                    [ThemeToggle] |  <- Header (sticky)
+------------------------------------------+
|                                          |
|              Main Content                |  <- Scrollable
|                                          |
+------------------------------------------+
|  Footer (minimal - links, version)       |  <- Footer
+------------------------------------------+
```

## Deliverables

- [ ] Design tokens defined in app.css
- [ ] Dark/light mode toggle works
- [ ] Theme persists across page reload
- [ ] All base components render correctly
- [ ] App shell displays with header/footer
- [ ] Components work in both themes
- [ ] No layout shift on theme change
