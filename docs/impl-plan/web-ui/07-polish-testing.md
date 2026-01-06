# Phase 7: Polish & Testing

**Status**: Completed
**Duration**: ~1 day
**Depends on**: Phase 6 (Results Display)

## Overview

Final polish pass: error states, loading skeletons, accessibility audit, keyboard navigation, and comprehensive tests. Ship-ready quality.

## Tasks

| Task | Description |
|------|-------------|
| Build ErrorState component | Friendly error display with retry |
| Build EmptyState component | No results messaging |
| Build Skeleton components | Loading placeholders |
| Add loading states | All async operations show skeletons |
| Accessibility audit | ARIA labels, focus management, contrast |
| Keyboard navigation | Full app navigable without mouse |
| Write component tests | Vitest + Testing Library |
| Write integration tests | Full flows with mocked API |
| Add page metadata | Title, description, OG tags |
| Performance check | Lighthouse audit, fix issues |
| Add favicon | App icon |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `web/src/lib/components/ErrorState.svelte` | **New file** - Error display |
| `web/src/lib/components/EmptyState.svelte` | **New file** - Empty/no results |
| `web/src/lib/components/Skeleton.svelte` | **New file** - Loading placeholder |
| `web/src/lib/components/AnswerSkeleton.svelte` | **New file** - Answer loading state |
| `web/src/lib/components/SourceSkeleton.svelte` | **New file** - Source loading state |
| `web/src/app.html` | **Modify** - Meta tags, favicon |
| `web/static/favicon.svg` | **New file** - App icon |
| `web/src/routes/+layout.svelte` | **Modify** - Error boundary |
| `web/src/lib/components/*.test.ts` | **New files** - Component tests |
| `web/src/lib/api/*.test.ts` | **Modify** - Expand API tests |
| `web/tests/integration/*.test.ts` | **New files** - Flow tests |

## Error States

| Error | Message | Action |
|-------|---------|--------|
| Network error | "Couldn't connect to server" | Retry button |
| Job not found | "Research job not found" | New research button |
| Rate limited | "Too many requests" | Show wait time |
| Search failed | "Search providers unavailable" | Retry button |
| LLM failed | "Couldn't generate answer" | Retry button |
| Generic | "Something went wrong" | Retry + report link |

## Skeleton Patterns

```svelte
<!-- Answer skeleton -->
<div class="animate-pulse">
  <div class="h-6 bg-gray-200 rounded w-3/4 mb-4"></div>
  <div class="h-4 bg-gray-200 rounded w-full mb-2"></div>
  <div class="h-4 bg-gray-200 rounded w-full mb-2"></div>
  <div class="h-4 bg-gray-200 rounded w-2/3"></div>
</div>
```

## Accessibility Checklist

- [ ] All interactive elements focusable
- [ ] Focus visible indicator (outline)
- [ ] Skip to main content link
- [ ] ARIA labels on icon-only buttons
- [ ] ARIA live region for streaming updates
- [ ] Color contrast meets WCAG AA
- [ ] Reduced motion support
- [ ] Screen reader tested (VoiceOver)

## Keyboard Navigation

| Key | Action |
|-----|--------|
| Tab | Move focus forward |
| Shift+Tab | Move focus backward |
| Enter | Activate button/link |
| Escape | Close modal/panel |
| Ctrl+Enter | Submit query |
| / | Focus search input (from anywhere) |

## Test Coverage Targets

| Area | Coverage |
|------|----------|
| API client | 90%+ |
| Components | 80%+ |
| Store logic | 90%+ |
| Integration flows | Key paths |

## Test Structure

```typescript
// Component test example
import { render, screen } from '@testing-library/svelte'
import { describe, it, expect } from 'vitest'
import Button from './Button.svelte'

describe('Button', () => {
  it('renders with correct text', () => {
    render(Button, { props: { children: 'Click me' } })
    expect(screen.getByRole('button')).toHaveTextContent('Click me')
  })

  it('shows loading state', () => {
    render(Button, { props: { loading: true } })
    expect(screen.getByRole('button')).toBeDisabled()
  })
})
```

## Page Metadata

```html
<svelte:head>
  <title>gorkd - Research Assistant</title>
  <meta name="description" content="Ask a question, get a cited answer" />
  <meta property="og:title" content="gorkd" />
  <meta property="og:description" content="Truth-seeking research bot" />
</svelte:head>
```

## Performance Targets

| Metric | Target |
|--------|--------|
| First Contentful Paint | < 1.5s |
| Largest Contentful Paint | < 2.5s |
| Cumulative Layout Shift | < 0.1 |
| Time to Interactive | < 3s |

## Deliverables

- [x] Error states display for all error types
- [x] Skeletons show during all loading states
- [x] Tab navigation works through entire app
- [x] Focus indicators visible
- [x] ARIA labels on all icon buttons
- [ ] Color contrast passes WCAG AA (manual audit needed)
- [x] Component tests passing (61 tests)
- [x] Integration tests for submit -> results flow
- [ ] Lighthouse performance score > 90 (manual audit needed)
- [x] Favicon displays in browser tab
- [x] OG tags render correctly
