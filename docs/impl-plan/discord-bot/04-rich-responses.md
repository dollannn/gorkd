# Phase 4: Rich Responses

**Status**: Planned
**Duration**: ~1 day
**Depends on**: Phase 3

## Overview

Transform plain text responses into rich Discord embeds with proper formatting, confidence indicators, source citations, and interactive buttons. Implement "View Full Analysis", "Show All Sources" (ephemeral), and "Retry" functionality.

## Tasks

| Task | Description |
|------|-------------|
| Success embed | Format completed research as embed |
| Error embed | Format failures with helpful messages |
| Confidence indicator | Color-coded confidence level |
| Source list | Top 3 sources with links |
| Action buttons | View Full, Show Sources, Retry |
| Button handlers | Handle button click interactions |
| Ephemeral sources | "Show All Sources" as ephemeral message |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-bot-discord/src/format/mod.rs` | **New file** - Format module |
| `crates/gorkd-bot-discord/src/format/embed.rs` | **New file** - Embed builders |
| `crates/gorkd-bot-discord/src/format/sources.rs` | **New file** - Source formatting |
| `crates/gorkd-bot-discord/src/interactions/mod.rs` | **New file** - Interaction module |
| `crates/gorkd-bot-discord/src/interactions/buttons.rs` | **New file** - Button handlers |
| `crates/gorkd-bot-discord/src/handler.rs` | Route button interactions |

## Implementation Details

### Success Embed Structure

```
┌────────────────────────────────────────────────┐
│ Research Complete                    [Green]   │
├────────────────────────────────────────────────┤
│                                                │
│ **Question**                                   │
│ What caused the CrowdStrike outage?            │
│                                                │
│ **Answer**                                     │
│ The July 2024 CrowdStrike outage was caused... │
│                                                │
│ **Confidence**: High (4 sources)               │
│                                                │
│ **Key Sources**                                │
│ 1. [Microsoft Blog](url) - Official response   │
│ 2. [CrowdStrike](url) - Incident report        │
│ 3. [Reuters](url) - Impact analysis            │
│                                                │
├────────────────────────────────────────────────┤
│ 12.3s | 8 sources analyzed                     │
└────────────────────────────────────────────────┘
│ [View Full] [Show Sources] [Share]             │
```

### Embed Colors by Confidence

| Confidence | Color | Hex |
|------------|-------|-----|
| High | Green | `#2ECC71` |
| Medium | Yellow | `#F1C40F` |
| Low | Orange | `#E67E22` |
| Insufficient | Red | `#E74C3C` |

### Button Custom IDs

| Button | Custom ID Format | Action |
|--------|------------------|--------|
| View Full Analysis | `view_full:{job_id}` | Link to web UI |
| Show All Sources | `show_sources:{job_id}` | Ephemeral with full list |
| Retry | `retry:{job_id}` | Re-run research |
| Share | `share:{job_id}` | Copy shareable link |

### Ephemeral Sources Message

When "Show All Sources" clicked:
1. Parse job_id from custom_id
2. Fetch sources from API
3. Format as numbered list with relevance scores
4. Respond with ephemeral message (only visible to clicker)

### Error States

| Error Type | Embed Title | Color |
|------------|-------------|-------|
| No sources found | "Insufficient Information" | Yellow |
| Search failed | "Research Failed" | Red |
| Timeout | "Request Timed Out" | Orange |
| API unavailable | "Service Unavailable" | Red |

## Key Considerations

- Discord embed description limit: 4096 characters
- Truncate long answers with "... [View Full]"
- Source URLs must be valid or Discord rejects embed
- Button custom_id max length: 100 characters
- Ephemeral messages can't be edited after sent

## Deliverables

- [ ] Success embed matches spec in discord.md
- [ ] Error embeds for all failure modes
- [ ] Confidence color-coding works
- [ ] "View Full Analysis" links to web UI
- [ ] "Show All Sources" shows ephemeral list
- [ ] "Retry" re-runs the research
- [ ] Long answers truncated properly
- [ ] Embed renders correctly on mobile
