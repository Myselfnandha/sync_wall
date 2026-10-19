# Architecture Decisions & Invariants

This document records durable architectural decisions, system invariants, and interface contracts for this project.

---

## Active Invariants

### 1. Dual-Layer Codebase Analysis Engine
- **Decision**: Codebase freshness is checked via `analyze.py --check --auto-heal --inject-ki` at the start of every session.
- **Rationale**: Prevents stale context without blocking user conversations, storing SQLite cache in `.agents/cache/codebase.db`.

### 2. Multi-Tier Security & Verification Gates
- **Decision**: All modified files and business logic must pass AAA unit test standards and Skylos SAST / AI hallucination verification.
- **Rationale**: Eliminates phantom imports, hallucinated package versions, and insecure sinks before commit.

### 3. Native MCP Server Protocol
- **Decision**: External tool integrations use the official Model Context Protocol (MCP) in `.agents/mcp_config.json`.

### 4. Reactive Async Task Notification & Live Status
- **Decision**: Always use the reactive "Waiting for a task notification" approach with live status rather than polling loops or redundant timers.
- **Rationale**: The IDE event loop automatically resumes execution when background tasks complete. Polling or using blind sleep/timers wastes tokens, introduces latency, and adds noise.

### 5. Single-File Analysis Whole-Read Invariant
- **Decision**: When analyzing a single file, read the entire file in a single tool call without piecemeal slicing.
- **Rationale**: Prevents fragmented code context, eliminates repeated round-trips for contiguous lines, and guarantees complete semantic understanding of the file.

### 6. Pre-Execution Tool Optimization & Context Economy
- **Decision**: Prior to triggering any action, assess the action against available tools to choose the most token-efficient, fast, and direct execution path.
- **Rationale**: Using the best-fit tool (dedicated search/edit/read/async primitives over shell commands or fragmented multi-step calls) preserves context window, eliminates round-trip latency, and prevents hallucination.



