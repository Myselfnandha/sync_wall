---
trigger: always_on
---

# §9 EFFORT — Model Reasoning Effort Override

Active effort level controls 5 dimensions of model behavior. Default: `high` (Claude-style deep reasoning). Override anytime via `/effort-<level>`.

| Dimension              | `low`                                   | `mid`                                    | `high` (default)                          | `ultra`                                     |
|------------------------|----------------------------------------|------------------------------------------|-------------------------------------------|---------------------------------------------|
| **Reasoning depth**    | Act immediately. Max 1 file read.      | 2-5 file reads. Check KI summaries.      | Full context sweep. Read all relevant KI.  | Exhaustive. Read every related file/doc.     |
| **Response verbosity** | Conclusion-only. 1-3 lines max.        | Conclusion-first + key reasoning. 3-8 lines. | Full reasoning chain. Show trade-offs.    | Detailed report with alternatives & risks.   |
| **Planning threshold** | Never create plans. Just execute.      | Plan only for multi-file architectural changes. | Plan for any non-trivial change. Show blast radius. | Always plan. Full implementation_plan.md + walkthrough.md. |
| **Verification rigor** | Trust the edit. No test runs.          | Run tests if test suite exists.          | Run tests + lint + build verification.     | Full test suite + Skylos gate + manual check list. |
| **Research breadth**   | Use only what's in immediate context.  | Check KI + grep relevant files.          | KI + grep + read docs + check web if needed. | KI + grep + docs + web + cross-reference everything. |

**Rules**: Override anytime with `/effort-low`, `/effort-mid`, `/effort-high`, or `/effort-ultra`. Stacks with any active workflow. Announce once on override: `⚡ Effort: <level>`. Default resets to `high`.
