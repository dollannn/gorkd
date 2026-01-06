# Discord Bot

Discord interface for gorkd research.

## Interaction Model

```
User: @gorkd What caused the CrowdStrike outage?
       │
       ▼
Bot:  🔍 Researching... (initial response, <3s)
       │
       ▼
Bot:  [Updates message with progress]
       │
       ▼
Bot:  📊 Research Complete (embed with answer)
```

## Commands

### Mention Query

Mention the bot with a question.

```
@gorkd What is the current status of the Mars Perseverance rover?
```

### Slash Command

```
/research query:What is the current status of the Mars Perseverance rover?
```

### Context Menu

Right-click a message → Apps → "Research This"

Useful for fact-checking claims in existing messages.

## Response Format

### Initial Acknowledgment (<3 seconds)

```
🔍 Researching your question...

Query: "What caused the CrowdStrike outage?"
```

### Progress Update

```
🔍 Researching your question...

Query: "What caused the CrowdStrike outage?"

Status: Searching sources...
Found: 8 relevant sources
```

### Final Response (Embed)

```
┌────────────────────────────────────────────────────────┐
│ 📊 Research Complete                                    │
├────────────────────────────────────────────────────────┤
│                                                         │
│ **Question**                                            │
│ What caused the CrowdStrike outage?                    │
│                                                         │
│ **Answer**                                              │
│ The July 2024 CrowdStrike outage was caused by a       │
│ faulty content update to the Falcon sensor software.   │
│ The update caused Windows systems to crash with a      │
│ blue screen, affecting approximately 8.5 million       │
│ devices worldwide.                                      │
│                                                         │
│ **Confidence**: 🟢 High (4 corroborating sources)      │
│                                                         │
│ **Key Sources**                                         │
│ 1. [Microsoft Blog](https://...) - Official response   │
│ 2. [CrowdStrike](https://...) - Incident report        │
│ 3. [Reuters](https://...) - Impact analysis            │
│                                                         │
│ ⏱️ 12.3s • 📚 8 sources analyzed                        │
├────────────────────────────────────────────────────────┤
│ [View Full Analysis] [Show All Sources] [Share]        │
└────────────────────────────────────────────────────────┘
```

## Buttons

### View Full Analysis

Opens web UI with complete results:
- Full answer with all details
- Complete citation list
- Source content previews
- Research metadata

### Show All Sources

Expands to show all sources (ephemeral message):

```
📚 All Sources for "What caused the CrowdStrike outage?"

1. Microsoft Blog (microsoft.com) - Relevance: 95%
   "Helping our customers through the CrowdStrike outage"
   Published: July 20, 2024

2. CrowdStrike Blog (crowdstrike.com) - Relevance: 94%
   "Technical Details: Falcon Content Update"
   Published: July 20, 2024

[... more sources ...]

💡 Click a source to view in browser
```

### Share

Generates a shareable link to the web UI result.

## Error States

### No Results

```
┌────────────────────────────────────────────────────────┐
│ ⚠️ Insufficient Information                            │
├────────────────────────────────────────────────────────┤
│                                                         │
│ **Question**                                            │
│ What will the stock market do tomorrow?                │
│                                                         │
│ I couldn't find reliable sources to answer this        │
│ question. This might be because:                       │
│                                                         │
│ • The question asks for future predictions             │
│ • No credible sources cover this topic                 │
│ • The topic is too recent for indexed sources          │
│                                                         │
│ **Suggestion**: Try rephrasing or asking about         │
│ historical data instead.                               │
│                                                         │
└────────────────────────────────────────────────────────┘
```

### Search Failed

```
┌────────────────────────────────────────────────────────┐
│ ❌ Research Failed                                      │
├────────────────────────────────────────────────────────┤
│                                                         │
│ Unable to complete research due to a technical issue.  │
│                                                         │
│ Error: Search providers temporarily unavailable        │
│                                                         │
│ [Retry]                                                │
│                                                         │
└────────────────────────────────────────────────────────┘
```

## Threading

- Initial response in channel where mentioned
- Follow-up questions in thread
- "Show All Sources" as ephemeral (only visible to requester)
- Long answers auto-thread if channel is busy

## Rate Limiting

| Limit | Value |
|-------|-------|
| Per user | 5 queries/minute |
| Per server | 20 queries/minute |
| Cooldown message | "Please wait {n} seconds before your next query" |

## Permissions Required

- Send Messages
- Embed Links
- Use External Emojis
- Add Reactions
- Read Message History (for context menu)

## Configuration (Server Admins)

Future: Server-specific settings via `/gorkd config`

- Allowed channels
- Default response visibility
- Rate limit overrides
