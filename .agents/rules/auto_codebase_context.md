---
trigger: always_on
---

# Auto Codebase Context (Global)

For any workspace containing a `.agent/` or `.agents/` directory:

1. **On conversation start**, silently check codebase freshness and auto-heal:
   ```
   python .agents/scripts/analyze.py . --check --auto-heal --inject-ki 2>/dev/null || python .agent/scripts/analyze.py . --check --auto-heal --inject-ki
   ```

2. **Result**: Every new conversation starts with fresh codebase knowledge in persistent memory.

> This rule applies globally to ALL workspaces that have `.agent/scripts/analyze.py` or `.agents/scripts/analyze.py`.
> The KI is stored per-project in `~/.gemini/antigravity-ide/knowledge/codebase-context-<project>/`.
