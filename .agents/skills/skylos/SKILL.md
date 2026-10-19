---
name: skylos
description: Local-first static analysis, dead code detection, security scanning, secrets auditing, and deterministic AI-code hallucination verification. Use to run CLI scans or invoke Skylos MCP tools.
license: Apache-2.0
metadata:
  tags: "Security, SAST, Dead Code, AI Verification, MCP, Static Analysis"
  category: "quality"
---

# Skylos Static Analysis & AI Verification

Skylos is a local-first static analysis engine and PR gate for Python, TypeScript/JavaScript, Go, Java, Kotlin, PHP, Rust, Dart, C#, and Shell.

## Key Capabilities

1. **Dead Code & Unused Elements**: Detects unused functions, classes, imports, routes, and package entrypoints (`skylos .`).
2. **Security & Vulnerabilities**: Catches injection flaws (SQL, XSS, SSRF, command), dangerous dataflows, and unsafe deserialization (`skylos . -a`).
3. **Secrets Detection**: High-entropy strings, tokens, and private credentials.
4. **AI-Code Hallucination Verification**: Verifies generated code against workspace ASTs for invented helpers, broken imports, missing guards, and dependency hallucinations (`skylos verify . --file <path>`).
5. **Pre-deployment Agent Guardrails**: Verifies LLM tools, sinks, prompt injection risks, and OWASP LLM coverage (`skylos defend .`).

---

## MCP Server Tools

When registered in `mcp_config.json`, the Skylos MCP server exposes:

| MCP Tool | Description |
|:---|:---|
| `verify_change` | Verifies a file or line range for AI-code defects and hallucinated APIs against project context. |
| `validate_code_change` | Inspects unified diffs for security regressions, secrets, and hallucinated packages against registries. |
| `verify_agent` | Statically scores and checks agent guardrails (prompt injection, tool permissions, PII, budget). |
| `security_scan` | Scans workspace for high/medium security vulnerabilities and dangerous sinks. |
| `quality_check` | Analyzes complexity, deep nesting, duplicate branches, and maintainability. |
| `architecture_check` | Validates dependency graphs and module coupling. |
| `health_score` | Computes a composite repository health score across dead code, security, and quality. |
| `generate_fix` | Produces verified dead-code removal plans and unified diffs. |

---

## Common CLI Commands

```bash
# Dead code scan
skylos .

# Full security, quality, secrets & AI defect audit
skylos . -a

# AI code verification on a specific edited file
skylos verify . --file src/app.py --project-context

# Safe import/function dead-code preview
skylos clean . --dry-run --confidence 80

# Review active git diff against main
skylos . -a --diff origin/main
```
