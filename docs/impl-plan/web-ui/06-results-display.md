# Phase 6: Results Display

**Status**: Planned
**Duration**: ~1-1.5 days
**Depends on**: Phase 5 (SSE Streaming)

## Overview

Build the results view showing the research answer with inline citations, confidence indicator, source cards, and metadata. The core value delivery screen.

## Tasks

| Task | Description |
|------|-------------|
| Build AnswerCard component | Summary + detail with citations |
| Build CitationLink component | Inline citation with hover preview |
| Build ConfidenceBadge component | High/medium/low/insufficient indicator |
| Build LimitationsList component | Answer limitations display |
| Build SourceCard component | Full source card with content preview |
| Build SourcesPanel component | Collapsible sources list |
| Build MetadataBar component | Duration, sources count, cached flag |
| Build ShareButton component | Copy link to clipboard |
| Build NewQueryButton component | Start new research |
| Integrate components | Full results page layout |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `web/src/lib/components/AnswerCard.svelte` | **New file** - Main answer display |
| `web/src/lib/components/CitationLink.svelte` | **New file** - Inline citation |
| `web/src/lib/components/ConfidenceBadge.svelte` | **New file** - Confidence indicator |
| `web/src/lib/components/LimitationsList.svelte` | **New file** - Caveats list |
| `web/src/lib/components/SourceCard.svelte` | **New file** - Source detail card |
| `web/src/lib/components/SourcesPanel.svelte` | **New file** - Sources container |
| `web/src/lib/components/MetadataBar.svelte` | **New file** - Job metadata |
| `web/src/lib/components/ShareButton.svelte` | **New file** - Share functionality |
| `web/src/lib/components/NewQueryButton.svelte` | **New file** - New research CTA |
| `web/src/routes/research/[id]/+page.svelte` | **Modify** - Full results layout |

## Results Page Layout

```
+----------------------------------------------------------+
| [< New Research]                    [Share] [ThemeToggle] |
+----------------------------------------------------------+
| Query: "What caused the 2024 CrowdStrike outage?"        |
+----------------------------------------------------------+
|                                                           |
| +-------------------------------------------------------+ |
| | ANSWER                               [High Confidence] | |
| |                                                        | |
| | The July 2024 CrowdStrike outage was caused by a      | |
| | faulty content update to the Falcon sensor [1].       | |
| | Microsoft estimates 8.5 million devices affected [2]. | |
| |                                                        | |
| | [Read more v]                                          | |
| +-------------------------------------------------------+ |
|                                                           |
| +-------------------------------------------------------+ |
| | LIMITATIONS                                            | |
| | - Technical root cause analysis still ongoing          | |
| | - Full financial impact not yet calculated             | |
| +-------------------------------------------------------+ |
|                                                           |
| +-------------------------------------------------------+ |
| | SOURCES (5)                                    [v]     | |
| | +---------------------------------------------------+ | |
| | | [1] microsoft.com                    0.95 relevance| | |
| | | Helping our customers through the CrowdStrike...  | | |
| | +---------------------------------------------------+ | |
| | | [2] crowdstrike.com                  0.92 relevance| | |
| | | Falcon Content Update Remediation...              | | |
| | +---------------------------------------------------+ | |
| +-------------------------------------------------------+ |
|                                                           |
| Completed in 14.2s | 12 sources considered | Not cached   |
+----------------------------------------------------------+
```

## Citation Interaction

Citations appear as `[1]` links in text:
- Hover: Show source title + domain in tooltip
- Click: Scroll to source in panel + highlight

## Confidence Badge Styles

| Level | Color | Icon |
|-------|-------|------|
| High | Green | CheckCircle |
| Medium | Yellow | AlertCircle |
| Low | Orange | AlertTriangle |
| Insufficient | Red | XCircle |

## Source Card Content

- Favicon (via Google favicon service or fallback)
- Domain name
- Title
- Published date (if available)
- Relevance score (progress bar)
- Content preview (first ~200 chars)
- "Used in answer" badge if cited
- External link icon

## Key Considerations

- Answer may be long - use expandable "Read more"
- Citations must be keyboard navigable (a11y)
- Sources panel collapsible on mobile
- Handle missing optional fields gracefully
- Preserve scroll position when expanding/collapsing
- Favicon fallback for domains without one

## Mobile Adaptations

- Sources panel becomes full-width accordion
- Citation tooltips become bottom sheets
- Metadata bar stacks vertically

## Deliverables

- [ ] Answer displays with formatted citations
- [ ] Citation hover shows source preview
- [ ] Citation click scrolls to source
- [ ] Confidence badge shows correct level/color
- [ ] Limitations list renders if present
- [ ] Sources panel shows all sources
- [ ] Sources indicate if used in citations
- [ ] Metadata bar shows duration/count/cached
- [ ] Share button copies URL to clipboard
- [ ] New Research button returns to home
- [ ] Layout works on mobile (responsive)
