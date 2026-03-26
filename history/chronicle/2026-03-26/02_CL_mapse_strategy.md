# Strategy Proposal: Trenza MAPSE Development Path

**Date:** 2026-03-26
**Author:** CL (Claude Opus 4.6)
**Status:** PROPOSAL — requires review by César and GE

---

## 0. What we are building

A **Minimal Ada-style Programming Support Environment (MAPSE)** for Trenza, where:
- The primary input modality is **conversation** (voice or text)
- The AI agents generate and iterate on .trz specifications
- The compiler verifies formally and returns feedback
- The human never needs to read or write code directly
- The system is accessible to visually impaired users by design

---

## 1. Architecture: Three Layers

### Layer 1: VS Code as the Build Host (KAPSE)
VS Code is not the user interface — it is the **build infrastructure**. It provides:
- Terminal access (cargo, npm, wasm-pack)
- File system management
- Git integration
- Extension API for diagnostics, language server, etc.
- Existing AI extensions (Claude Code, Gemini Code Assist) as execution engines

The user does not need to look at VS Code. VS Code is the machine room.

### Layer 2: The Trenza Extension as Orchestrator (MAPSE core)
The VS Code extension becomes the **bridge** between conversation and compilation:

```
Voice/Text → AI Agent → .trz generation → Compiler → Diagnostics → Voice/Text feedback
                ↑                                          |
                └──────────── iteration loop ───────────────┘
```

**Key capabilities needed:**
1. Receive intent from a conversation (text or voice transcription)
2. Invoke the appropriate AI agent (Claude or Gemini) to generate/modify .trz
3. Run the Trenza compiler and capture diagnostics
4. Return results in a format suitable for voice synthesis
5. Manage the iteration loop (intent → spec → verify → refine)

### Layer 3: The Conversation Interface (MAPSE front-end)
Multiple possible front-ends, all consuming the same orchestrator:
- **VS Code chat panel** (immediate, for sighted users during development)
- **Mobile app via Dispatch** (Claude) or equivalent (Gemini)
- **Web interface** (the WASM demo GE built, extended with voice)
- **Standalone voice client** (future: phone call, smart speaker)

---

## 2. Development Phases

### Phase 1: Validate within VS Code (weeks 1-4)
**Goal:** Prove that a conversation in VS Code can produce a compiled .trz with
zero manual file editing.

**Approach:**
- Use the existing Trenza VS Code extension for diagnostics
- Use Claude Code (already in VS Code) or Gemini Code Assist as the AI agent
- Build a simple **Trenza Skill** (Claude Code skill or equivalent) that:
  1. Accepts natural language intent
  2. Generates a .trz file
  3. Invokes the compiler
  4. Returns pass/fail with human-readable explanation
- Test with CronometroPSP: can we recreate one overlay purely from conversation?

**Success criterion:** A recorded session where a user describes an overlay in
natural language and gets a compiled, verified .trz without opening a file.

**What GE should build:**
- Ensure `--out-dir` works (already planned)
- Expose compiler diagnostics as JSON (machine-readable, not just stderr)
- Ensure the VS Code extension can invoke the compiler programmatically

**What CL should build:**
- A Claude Code skill (`/trenza` command) that orchestrates the flow
- Prompt engineering for natural-language-to-.trz conversion
- Feedback formatting for voice-compatible output

### Phase 2: Add voice (weeks 5-8)
**Goal:** Replace text input with voice input within VS Code.

**Approach:**
- VS Code has a Speech API (`vscode.speech`) since late 2024
- Integrate speech-to-text as input to the Trenza skill
- Integrate text-to-speech for compiler feedback
- The AI agent still runs as text (STT → agent → TTS), but the user speaks

**Success criterion:** A user dictates an overlay specification and hears
"Specification valid. 2 states, 3 transitions, all 8 rules pass."

### Phase 3: Mobile bridge (weeks 9-12)
**Goal:** Control the Trenza build environment from a phone.

**Approach:**
- If using Claude: Dispatch already bridges phone → desktop
- If using Gemini: evaluate Antigravity mobile capabilities
- The Trenza skill on the desktop receives instructions from the mobile bridge
- Voice on phone → STT → intent → desktop agent → .trz → compiler → TTS → phone

**Success criterion:** César dictates a specification from his phone, the desktop
compiles it, and he hears the result on his phone.

### Phase 4: Standalone demo (weeks 13+)
**Goal:** A public demo that anyone can try.

**Approach:**
- Web-based interface using the WASM compiler (GE's demo as starting point)
- Browser Speech API for voice input/output
- A cloud-hosted AI agent (API call) for .trz generation
- No VS Code required — pure browser experience

---

## 3. The MCP Server for Agent Coordination

### Problem
Currently, agents coordinate via chronicle entries and LOCK.md — a process that
works but consumes tokens (reading/writing markdown) and is slow (requires git
push/pull cycles). As we add voice interaction and real-time compilation, we need
faster coordination.

### Proposed solution: `trenza-coord` MCP server

A lightweight MCP server running locally that provides:

1. **Lock management** — acquire/release/query locks without file I/O
2. **Agent presence** — which agents are active, what they're working on
3. **Message passing** — short messages between agents without chronicle overhead
4. **Compiler invocation** — any agent can trigger compilation and get results
5. **Session state** — shared key-value store for the current session

**Implementation:**
- Rust binary (consistent with the rest of the toolchain)
- Runs on localhost, listens on a Unix socket or TCP port
- MCP protocol over stdio (standard MCP transport)
- State persisted to a simple SQLite database
- On session close, relevant state is flushed to chronicle (audit trail)

**Security:**
- Local-only by default (no network exposure)
- Agent authentication via session tokens (generated at startup)
- Message signing with Ed25519 (simple, fast, quantum-resistant is unnecessary
  for local coordination — but if César insists on post-quantum, we can use
  CRYSTALS-Dilithium for signatures, which is NIST-standardized as ML-DSA)
- All state is auditable (SQLite WAL + flush to chronicle)

**Token savings:**
- Lock check: 0 tokens (local query) vs. ~500 tokens (read LOCK.md)
- Brief message: 0 tokens (local) vs. ~1000 tokens (write + read chronicle entry)
- Compiler invocation: 0 tokens for protocol overhead
- Estimated saving: 20-30% of coordination overhead per session

**MCP tools exposed:**
```
trenza_lock_acquire(area, agent, task)
trenza_lock_release(area, agent)
trenza_lock_query()
trenza_agent_register(agent_id, capabilities)
trenza_agent_heartbeat(agent_id)
trenza_message_send(from, to, content)
trenza_message_poll(agent_id)
trenza_compile(input_files, out_dir, targets)
trenza_state_get(key)
trenza_state_set(key, value)
```

### Why not a cloud MCP?
- Latency: local is <1ms, cloud adds 50-200ms per call
- Cost: zero vs. API costs
- Privacy: all data stays on César's machine
- Simplicity: no auth, no deployment, no billing

---

## 4. Local Models: Recommended Configuration

### Rationale
Claude (Opus/Sonnet) and Gemini (Pro/Flash) are expensive for routine tasks:
syntax checking, simple refactoring, commit message generation, documentation
formatting. A local model handles these without consuming API tokens.

### Recommended setup: Ollama

**Why Ollama (not Docker/vLLM/llama.cpp):**
- Simplest setup on Windows (native installer, runs as service)
- Automatic GPU detection and offloading (NVIDIA on César's machine?)
- Model management built in (`ollama pull`, `ollama list`)
- OpenAI-compatible API on localhost:11434
- MCP server available (`ollama-mcp-server`)

**Recommended models (depends on available VRAM):**

| Model | Size | VRAM | Role | Tasks |
|-------|------|------|------|-------|
| **Qwen3 8B** | ~5GB | 6GB | Code assistant | .trz syntax help, Rust/TS snippets, commit messages |
| **DeepSeek-Coder-V2-Lite 16B** | ~10GB | 10GB | Code specialist | Larger refactors, test generation, documentation |
| **Phi-4 14B** | ~9GB | 10GB | General reasoning | Chronicle summaries, briefing drafts, review prep |

**If César's machine has 16GB+ VRAM:**
Add **Codestral 22B** (Mistral) — strongest open coding model, can handle
Rust compilation errors and suggest fixes autonomously.

**If limited to 8GB VRAM:**
Use only **Qwen3 8B** — surprisingly capable for its size, handles most
routine tasks.

### Integration with the project

The local model would be invoked by Claude/Gemini agents for subtasks:
- "Summarize this git diff for the chronicle" → local model
- "Format this compiler output as a diagnostic message" → local model
- "Generate a commit message for these changes" → local model
- "Check if this .trz fragment is syntactically valid" → local model (with
  the grammar loaded as context)

This is the "junior developer" model: we (Claude/Gemini) direct, the local
model executes routine work. It never makes architectural decisions.

### Configuration
```bash
# Install Ollama (Windows)
winget install Ollama.Ollama

# Pull recommended model
ollama pull qwen3:8b

# Test
ollama run qwen3:8b "Explain what a statechart is in one paragraph"

# Start as service (automatic on Windows install)
# API available at http://localhost:11434
```

### MCP integration
The `trenza-coord` MCP server (section 3) could proxy local model calls,
so Claude/Gemini agents invoke local models through the same MCP interface
they use for everything else:
```
trenza_local_model(prompt, model="qwen3:8b", max_tokens=500)
```

---

## 5. Priorities and Dependencies

```
                    ┌─────────────────┐
                    │   ONWARD! Paper  │ ← CRITICAL PATH (deadline Apr-May)
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
     ┌────────────┐  ┌────────────┐  ┌──────────┐
     │  --out-dir  │  │  Related   │  │  Paper   │
     │  (GE)      │  │  Work (CL) │  │  Draft   │
     └─────┬──────┘  └────────────┘  └──────────┘
           │
           ▼
  ┌─────────────────┐
  │  Phase 1: VS    │ ← MAPSE validation
  │  Code skill     │
  └────────┬────────┘
           │
     ┌─────┴──────┐
     ▼            ▼
┌─────────┐ ┌──────────┐
│ Ollama  │ │ MCP coord│ ← Infrastructure (parallel)
│ setup   │ │ server   │
└─────────┘ └──────────┘
           │
           ▼
  ┌─────────────────┐
  │  Phase 2: Voice │ ← Voice integration
  └────────┬────────┘
           │
           ▼
  ┌─────────────────┐
  │  Phase 3: Phone │ ← Mobile bridge
  └─────────────────┘
```

**Recommendation:** Paper first (2-3 weeks), Phase 1 in parallel where
possible (GE on --out-dir and diagnostics JSON, CL on skill design).
Ollama setup can happen any time — it's independent.

---

*This is a proposal. César decides priorities. GE should review and flag
any technical concerns or alternative approaches.*
