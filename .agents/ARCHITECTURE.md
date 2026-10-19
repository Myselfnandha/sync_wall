# ZeroGravity OS Architecture

> Autonomous Supercoding Engine for Google Antigravity

---

## 📋 Overview

ZeroGravity OS is a lightweight, high-performance operating framework designed natively for Google Antigravity. It powers autonomous coding, intelligent memory sweeps, on-demand MCP integration, and multi-model pairing.

### Core Architectural Pillars
1. **5-Stage Autonomous Supercoding Pipeline**:
   - `Memory Sweep` → `Blueprint` → `CodeCrush` → `AAA Test` → `Sync`
2. **Context & Time Economy (§0)**:
   - Tool-first optimization: ripgrep over shell search, whole-file analysis, atomic multi-replace, and zero polling loops.
3. **On-Demand Intelligence & MCP**:
   - Zero active MCP servers by default. 12 verified servers in local registry, with access to 2,600+ catalog servers enabled on demand.
4. **Persistent Mistake Immunization**:
   - Auto-evolving memory (`anti_patterns.json`, `gotchas.md`, `decisions.md`) to prevent repeating errors across sessions.
5. **AGY Staffer & Fast Pairing**:
   - Native integration with Google Antigravity CLI (`agy`) for fast research, independent reviews, and background implementation.
6. **Ponytail YAGNI Engine**:
   - Pragmatic, minimal engineering plugin enforcing standard library first, zero bloat, and the 7-step simplification ladder.

---

## 🏗️ Directory Structure

```plaintext
.agents/
├── AGENTS.md                 # Primary system contract and agent instructions
├── VERSION                   # OS Framework version
├── antigravity.json          # Antigravity runtime phases and discovery mapping
├── hooks.json                # Native Antigravity PreToolUse hook configuration
├── mcp_config.json           # Workspace active MCP configuration (zero-default)
├── skills_index.json         # Searchable registry of installed skills
├── rules/                    # Workspace behavioral rules & constraints
│   ├── core.md               # §0–§9 Time economy, invariants, and safety
│   ├── codebase_context.md   # Automatic KI codebase context loader
│   └── effort.md             # Model reasoning effort level override
├── skills/                   # Verified specialized skills
│   ├── agy-*                 # Antigravity CLI delegate personas (staffer, ask, reviewer, etc.)
│   ├── audit-agent           # iFixAi safety, alignment, and accuracy audit
│   ├── coding-interview      # Computer science and DSA tutor
│   ├── compress-tokens       # Token compression and memory distillation
│   ├── humanize-text         # Anti-slop editor preserving human voice
│   ├── react-components     # ReUI shadcn component and icon registry
│   └── skill-router          # Gateway to 1,450+ on-demand external skills
├── plugins/                  # Modular plugin extensions
│   └── ponytail/             # Pragmatic senior developer YAGNI engine
├── workflows/                # 20 Slash-command procedures
│   ├── build-feature.md      # Autonomous 5-stage feature pipeline
│   ├── brainstorm.md         # Structured brainstorming
│   ├── create.md             # Interactive application creator
│   ├── debug.md              # Systematic root-cause debugging
│   ├── deploy.md             # Pre-flight and deployment verification
│   ├── test.md               # AAA test generator and executor
│   ├── enhance.md            # Iterative feature enhancement
│   ├── design-ui.md          # UI/UX design workflow
│   ├── codebase-design.md    # Deep module architecture design
│   ├── domain-model.md       # Ubiquitous language & domain modeling
│   ├── orchestrate.md        # Multi-perspective coordinator
│   ├── focus.md              # High-density output mode
│   ├── humanize-text.md      # Writing refiner
│   ├── compress-tokens.md    # Token compressor
│   ├── effort.md             # Reasoning effort override (/effort low|mid|high|ultra|off)
│   ├── mcp.md                # On-demand MCP server manager
│   ├── skill.md              # Skill discovery gateway
│   ├── yagni-simplify.md     # Ponytail simplification workflow
│   ├── audit-agent.md        # Agent safety inspection
│   └── react-components.md   # ReUI component installer
├── memory/                   # Persistent agent memory & learning state
│   ├── decisions.md          # Architectural decisions log
│   ├── gotchas.md            # Pitfalls, edge cases, and runtime traps
│   ├── anti_patterns.json    # Machine-readable anti-pattern database
│   ├── goals.md              # Project goals and active milestones
│   └── session_context.md    # Session continuity state
├── mcp-registry/             # MCP server registry and catalog
│   ├── servers.json          # 12 verified zero-default servers
│   └── mcp_catalog.json      # 2,600+ searchable MCP servers
├── hooks/                    # Antigravity Doctor, policies, tests, and plugin builder
│   ├── antigravity-doctor.mjs # 6-phase runtime and release diagnostics
│   ├── validate-tool-call.mjs # Destructive command safety gate
│   ├── sync-mcp.mjs          # Safe MCP configuration synchronizer
│   ├── build-plugin.mjs      # Local Antigravity plugin packager
│   └── tests/                # Automated validation test suite
└── scripts/                  # Autonomous codebase analysis, cache, and CLI
    ├── zg                    # ZeroGravity bash CLI wrapper
    ├── analyze.py            # AST-level codebase summarizer & KI injector
    ├── mcp.py                # MCP lifecycle management CLI
    ├── auto_evolve.py        # Silent learning and mistake distillation
    └── db_cache.py           # SQLite hash cache for sub-second analysis
```

---

## ⚡ Runtime Integration Phases

```text
.agents/antigravity.json
        ↓
Phase 1: Discovery      ── rules/ + skills/ + workflows/ + plugins/
        ↓
Phase 2: MCP            ── zero-default mcp_config.json + mcp-registry/
        ↓
Phase 3: Hooks          ── hooks.json + validate-tool-call.mjs
        ↓
Phase 4: Orchestration  ── /build-feature + /orchestrate + memory sweep
        ↓
Phase 5: Plugin         ── build-plugin.mjs packaging
        ↓
Phase 6: Validation      ── antigravity-doctor.mjs + test suites
```

---

## 🧪 Verification & Health Checks

```bash
# Antigravity 6-Phase Diagnostic Doctor
node .agents/hooks/antigravity-doctor.mjs

# Automated Runtime Test Suite
node .agents/hooks/tests/antigravity.test.mjs

# Unit Test Suite
python3 -m unittest discover tests -v

# NPM Package CLI Smoke Test
node bin/cli.js --help
```
