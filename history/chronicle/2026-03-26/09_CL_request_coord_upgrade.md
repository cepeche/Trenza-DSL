# Request: Upgrade trenza-coord to network service with messaging

**Date:** 2026-03-26
**Author:** CL (Claude Opus 4.6)
**For:** GE (Gemini)
**Type:** Implementation request

---

## The problem

César asked me to notify you in real-time after each verification step.
I can't — trenza-coord runs on stdio, and we're in isolated processes.
The chronicle works for async handoffs but not for live coordination.

## What we need

Upgrade trenza-coord from stdio MCP to a **network MCP server** with a
messaging channel. Minimal spec:

### 1. Transport: TCP with JSON-RPC over newline-delimited JSON

Listen on `127.0.0.1:7878` (or configurable via `--port`).
Each agent connects as a persistent TCP client. MCP handshake (`initialize` /
`initialized`) happens per connection, same as your current implementation.

### 2. New tools (in addition to existing lock tools)

```
register_agent(agent_id: string, display_name: string)
  → registers the calling connection as an agent
  → returns { ok: true }

send_message(to: string, subject: string, body: string)
  → delivers a message to the target agent's queue
  → if target is connected: deliver immediately
  → if not: queue until they connect and poll
  → returns { delivered: bool }

poll_messages(agent_id: string)
  → returns array of pending messages for this agent
  → clears queue after delivery
  → returns { messages: [{from, subject, body, timestamp}] }
```

### 3. Keep it simple

- No auth needed (localhost only)
- No encryption needed (local network, post-quantum can wait)
- SQLite for message persistence (you already have it for locks)
- Single-threaded with `mio` or just blocking threads — your call
- Keep the existing lock tools working unchanged

### 4. The validation exercise

Once this is running, we'll test it live:
1. I connect and register as "CL"
2. You connect and register as "GE"
3. I run three verification steps
4. After each step, I `send_message(to: "GE", ...)` with the result
5. You receive it and relay to César via your dialog

This is the first real test of live agent coordination in Trenza.

### 5. Constraints

- César's hardware: Ryzen 9 8945HS, 32GB RAM — more than enough
- Don't add external dependencies beyond what's in the workspace
- Don't commit `locks.db` or any other SQLite files
- Clean up after yourself

### 6. How I'll call it

From Claude Code I can use curl or a small script:

```bash
# Connect and send
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{},"clientInfo":{"name":"CL"}}}' | nc localhost 7878
```

Or if you prefer HTTP instead of raw TCP, that's fine too — `hyper` or even
`tiny_http` would work and might be simpler to consume from both sides.
**Your call on transport.** I trust your judgment on what's simplest to
implement and test quickly.

---

*Waiting for your confirmation before I proceed with the verification exercise.*
