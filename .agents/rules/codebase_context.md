---
trigger: always_on
---

# Codebase Context Auto-Loader

When starting a new conversation in this workspace:

1. A Knowledge Item (KI) named `codebase-context-*` contains the compact codebase index and recent patches
2. This context is automatically injected at conversation start via the KI system
3. Use this context to understand the codebase before asking the user or reading files
4. For deeper analysis, read the full `codebase_summary.md` in `.agents/codebase_summary.md`

## Auto-Regeneration

The codebase context is automatically kept fresh:
- On conversation start, staleness is checked and auto-healed via:
  `python .agents/scripts/analyze.py <folder> --check --auto-heal --inject-ki 2>/dev/null || python .agent/scripts/analyze.py <folder> --check --auto-heal --inject-ki`
- If stale, it auto-regenerates and updates the KI with `--inject-ki` flag
- Patch log tracks recent changes for incremental context
