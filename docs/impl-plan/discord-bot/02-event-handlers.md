# Phase 2: Event Handlers

**Status**: Planned
**Duration**: ~1-2 days
**Depends on**: Phase 1

## Overview

Implement the three ways users can trigger research: mentioning the bot, using the `/research` slash command, and right-clicking a message to select "Research This" from the context menu. Each handler extracts the query and delegates to shared research logic.

## Tasks

| Task | Description |
|------|-------------|
| Mention handler | Parse `@gorkd <query>` from message content |
| Slash command | Register and handle `/research query:<text>` |
| Context menu | Register and handle "Research This" message command |
| Query extraction | Shared logic to clean and validate query text |
| Command registration | Register application commands on startup |

## Files to Create/Modify

| File | Changes |
|------|---------|
| `crates/gorkd-bot-discord/src/handler.rs` | Implement EventHandler trait methods |
| `crates/gorkd-bot-discord/src/commands/mod.rs` | **New file** - Command module |
| `crates/gorkd-bot-discord/src/commands/research.rs` | **New file** - /research slash command |
| `crates/gorkd-bot-discord/src/commands/menu.rs` | **New file** - Context menu command |
| `crates/gorkd-bot-discord/src/query.rs` | **New file** - Query extraction/validation |

## Implementation Details

### EventHandler Methods

| Method | Trigger | Action |
|--------|---------|--------|
| `ready` | Bot connected | Register slash commands, log ready |
| `message` | New message | Check for mention, extract query, start research |
| `interaction_create` | Slash/menu/button | Route to appropriate handler |

### Slash Command Definition

```
/research
├── name: "research"
├── description: "Research a question with cited sources"
└── options:
    └── query (String, required): "The question to research"
```

### Context Menu Definition

```
Message Command: "Research This"
├── type: Message
├── name: "Research This"
└── action: Use message content as query
```

### Query Extraction Logic

1. For mentions: Strip `<@BOT_ID>` prefix and trim
2. For slash command: Use `query` option value directly
3. For context menu: Use target message content
4. Validate: Non-empty, <= 2000 chars
5. Return error embed if invalid

### Command Registration

On `ready` event:
1. Check if commands already registered (cache in state)
2. If not, use `Command::set_global_commands()` 
3. Log registered command count

## Key Considerations

- `MESSAGE_CONTENT` intent required to read message text for mentions
- Slash commands work without message content intent
- Context menu shows in right-click > Apps menu
- Defer response immediately for slash/menu (3 second limit)
- Mentions can reply directly (no defer needed initially)

## Deliverables

- [ ] Bot responds to `@gorkd <query>` mentions
- [ ] `/research` slash command registered globally
- [ ] "Research This" appears in message context menu
- [ ] Invalid queries show helpful error message
- [ ] Commands persist across bot restarts
- [ ] `cargo test -p gorkd-bot-discord` passes (query extraction tests)
