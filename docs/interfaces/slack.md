# Slack Bot

Slack interface for gorkd research.

## Interaction Model

```
User: @gorkd What caused the CrowdStrike outage?
       │
       ▼
Bot:  🔍 Researching... (in thread, <3s)
       │
       ▼
Bot:  [Updates with progress]
       │
       ▼
Bot:  📊 Research Complete (blocks with answer)
```

## Commands

### App Mention

Mention the bot in any channel where it's added.

```
@gorkd What is the current inflation rate in the EU?
```

### Slash Command

```
/research What is the current inflation rate in the EU?
```

### Message Shortcut

Click "..." on any message → "Research This"

For fact-checking existing messages.

## Response Format

### Initial Acknowledgment (<3 seconds)

Uses Slack's native "Searching..." indicator where possible, otherwise:

```
🔍 Researching your question...
━━━━━━━━━━━━━━━━━━━━━━━━━━━
Query: "What caused the CrowdStrike outage?"
```

### Progress Updates (in thread)

```
🔍 Research in progress...
━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 Status: Analyzing sources
📚 Found: 8 relevant sources
⏱️ Elapsed: 5s
```

### Final Response (Block Kit)

```
┌─────────────────────────────────────────────────────────────┐
│ 📊 Research Complete                                         │
└─────────────────────────────────────────────────────────────┘

*Question*
What caused the CrowdStrike outage?

───────────────────────────────────────────────────────────────

*Answer*
The July 2024 CrowdStrike outage was caused by a faulty content 
update to the Falcon sensor software. The update caused Windows 
systems to crash with a blue screen, affecting approximately 
8.5 million devices worldwide.

───────────────────────────────────────────────────────────────

*Confidence*: 🟢 High (4 corroborating sources)

*Key Sources*
• <https://blogs.microsoft.com/...|Microsoft Blog> - Official response
• <https://crowdstrike.com/...|CrowdStrike> - Incident report  
• <https://reuters.com/...|Reuters> - Impact analysis

───────────────────────────────────────────────────────────────

⏱️ 12.3s  •  📚 8 sources analyzed

┌──────────────────┐ ┌──────────────────┐ ┌──────────────────┐
│ View Full Report │ │  Show Sources    │ │     Share        │
└──────────────────┘ └──────────────────┘ └──────────────────┘
```

## Block Kit Structure

```json
{
  "blocks": [
    {
      "type": "header",
      "text": {"type": "plain_text", "text": "📊 Research Complete"}
    },
    {
      "type": "section",
      "text": {"type": "mrkdwn", "text": "*Question*\nWhat caused..."}
    },
    {"type": "divider"},
    {
      "type": "section",
      "text": {"type": "mrkdwn", "text": "*Answer*\nThe July 2024..."}
    },
    {"type": "divider"},
    {
      "type": "context",
      "elements": [
        {"type": "mrkdwn", "text": "*Confidence*: 🟢 High"}
      ]
    },
    {
      "type": "section",
      "text": {"type": "mrkdwn", "text": "*Key Sources*\n• ..."}
    },
    {"type": "divider"},
    {
      "type": "context",
      "elements": [
        {"type": "mrkdwn", "text": "⏱️ 12.3s  •  📚 8 sources"}
      ]
    },
    {
      "type": "actions",
      "elements": [
        {
          "type": "button",
          "text": {"type": "plain_text", "text": "View Full Report"},
          "url": "https://gorkd.app/jobs/..."
        },
        {
          "type": "button", 
          "text": {"type": "plain_text", "text": "Show Sources"},
          "action_id": "show_sources"
        }
      ]
    }
  ]
}
```

## Threading Behavior

| Scenario | Behavior |
|----------|----------|
| Direct mention in channel | Reply in thread |
| Mention in existing thread | Reply in same thread |
| Slash command | Ephemeral initial, thread for result |
| Message shortcut | Thread on original message |

## Button Actions

### View Full Report

Opens web UI in browser with complete results.

### Show Sources

Posts expanded source list in thread (ephemeral):

```
📚 All Sources

1. *Microsoft Blog* (microsoft.com)
   "Helping our customers through the CrowdStrike outage"
   Published: July 20, 2024 • Relevance: 95%

2. *CrowdStrike Blog* (crowdstrike.com)
   "Technical Details: Falcon Content Update"
   Published: July 20, 2024 • Relevance: 94%

[View 6 more sources...]
```

### Share

Generates shareable link, posts to channel (with confirmation).

## Error States

### Insufficient Sources

```
┌─────────────────────────────────────────────────────────────┐
│ ⚠️ Insufficient Information                                  │
└─────────────────────────────────────────────────────────────┘

*Question*
What will happen to Bitcoin next week?

I couldn't find reliable sources to answer this question.

*Possible reasons:*
• Question asks for future predictions
• Topic too speculative for factual sources
• Insufficient indexed content

*Suggestion:* Try asking about historical trends or current data.
```

### Technical Error

```
┌─────────────────────────────────────────────────────────────┐
│ ❌ Research Failed                                           │
└─────────────────────────────────────────────────────────────┘

Unable to complete research due to a technical issue.

Error: Search service temporarily unavailable

┌──────────┐
│  Retry   │
└──────────┘
```

## Rate Limiting

| Limit | Value |
|-------|-------|
| Per user | 5 queries/minute |
| Per workspace | 30 queries/minute |
| Cooldown | Ephemeral message with wait time |

## Required Scopes

```
app_mentions:read    - Receive @mentions
chat:write           - Send messages
commands             - Slash commands
im:history           - DM context
channels:history     - Channel context (for shortcuts)
groups:history       - Private channel context
```

## Event Subscriptions

```
app_mention          - @gorkd mentions
message.im           - Direct messages
```

## Socket Mode vs HTTP

| Mode | When to Use |
|------|-------------|
| Socket Mode | Development, internal workspaces, behind firewall |
| HTTP | Production, public distribution, high scale |

Default: Socket Mode for simplicity.

## Installation Flow

1. User clicks "Add to Slack"
2. OAuth flow → workspace install
3. Bot added to #general (or selected channel)
4. Welcome message with usage instructions

## Workspace Configuration

Future: `/gorkd settings` command for admins

- Restrict to specific channels
- Set default response visibility
- Configure rate limits
- Enable/disable features
