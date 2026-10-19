# Codebase Patches: sync_wall

## 2026-10-19 14:23:26 (16 files changed)
- `Cargo.lock` — Lockfile manifest: 279 resolved dependencies
- `Cargo.toml` — TOML config: workspace
- `core/Cargo.toml` — Cargo.toml (Rust project)
- `core/src/crop.rs` — structs: CropEngine; enums: CropError; 4 functions
- `core/src/lib.rs` — structs: SyncWallEngine; enums: EngineError
- `core/src/model.rs` — structs: Hash, Hlc, DerivativeSpec; enums: Fit, OutFormat, DeviceKind
- `core/src/net.rs` — structs: DeviceIdentity, ServerState, HelloResponse; enums: NetError; 9 functions
- `core/src/store.rs` — structs: Store; enums: StoreError
- `docs/design_spec.md` — sync_wall Architecture & Engineering Specification (113 lines)
- `docs/protocol.md` — sync_wall Wire & Synchronization Protocol (v1) (131 lines)
- `linux/backend/Cargo.toml` — Cargo.toml (Rust project)
- `linux/backend/src/lib.rs` — structs: Output, XfconfBackend, MockBackend; enums: BackendError; traits: WallpaperBackend; 1 functions
- `linux/cli/Cargo.toml` — Cargo.toml (Rust project)
- `linux/cli/src/main.rs` — structs: Cli; enums: Commands; 4 functions
- `linux/daemon/Cargo.toml` — Cargo.toml (Rust project)
- `linux/daemon/src/main.rs` — structs: Args, RpcRequest, RpcResponse; 3 functions

## 2026-10-18 14:08:27 (13 files changed)
- `.github/ISSUE_TEMPLATE/knowledge_submission.yml` — YAML config: name, description, title, labels, body
- `.github/workflows/ci.yml` — GitHub Actions workflow
- `.github/workflows/ingest_knowledge.yml` — GitHub Actions workflow
- `.gitignore` — ============================================================================== Git Igno... (42 lines)
- `.npmignore` — Development files & scripts (27 lines)
- `CONTRIBUTING.md` — Contributing to Antigravity Supercoder OS (59 lines)
- `LICENSE` — Text file: 17 lines
- `Makefile` — ============================================================================== 🌌 ZeroGr... (55 lines)
- `README.md` — 🌌 ZeroGravity OS ## *Autonomous Supercoder OS • Zero-Default MCPs • 28 Workflows* (147 lines)
- `bin/cli.js` — functions: copyDirSync, main, parseArgs, handleMcpCommand
- `pack.sh` — Shell script (env); functions: print_banner, usage, check_cmd
- `package.json` — package.json
- `tests/test_all.py` — Tests for ScriptImportability, CLIHelpExits, JSONDataSchemas: 14 test functions/methods

## 2026-10-17 19:52:35 (6 files changed)
- `.npmignore` — Development files & scripts (27 lines)
- `CONTRIBUTING.md` — Contributing to Antigravity Supercoder OS (59 lines)
- `Makefile` — ============================================================================== 🌌 ZeroGr... (55 lines)
- `README.md` — 🌌 ZeroGravity OS ## *Autonomous Supercoder OS • Zero-Default MCPs • 28 Workflows* (152 lines)
- `bin/cli.js` — functions: printBanner, downloadFile, askQuestion, resolveSourcePayload
- `package.json` — package.json

## 2026-10-15 19:51:08 (5 files changed)
- `AGENTS.md` — ZeroGravity OS (AGENTS.md) (7 lines)
- `CONTRIBUTING.md` — Contributing to Antigravity Supercoder OS (60 lines)
- `openhands_config.toml` — TOML config: core, agent, llm
- `package.json` — package.json
- `tests/test_all.py` — Tests for ScriptImportability, CLIHelpExits, JSONDataSchemas: 14 test functions/methods

## 2026-10-14 13:25:47 (57 files changed)
- `.github/ISSUE_TEMPLATE/knowledge_submission.yml` — YAML config: name, description, title, labels, body
- `.github/skills/brandkit/SKILL.md` — - (30 lines)
- `.github/skills/design-system/SKILL.md` — - (79 lines)
- `.github/skills/design-system/data/chart-types.md` — Chart and data-viz database (52 lines)
- `.github/skills/design-system/data/layouts.md` — Layout archetypes (28 lines)
- `.github/skills/design-system/data/motion-presets.md` — Motion preset database (46 lines)
- `.github/skills/design-system/data/palettes.md` — Palette database (143 lines)
- `.github/skills/design-system/data/styles.md` — Style database (71 lines)
- `.github/skills/design-system/data/tech-stacks.md` — Tech-stack database (102 lines)
- `.github/skills/design-system/data/type-pairings.md` — Type-pairing database (80 lines)
- `.github/skills/design-system/reference/color.md` — Building a committed palette and the light/dark map (22 lines)
- `.github/skills/design-system/reference/dna.md` — House design DNA (32 lines)
- `.github/skills/design-system/reference/glass.md` — Liquid glass and surface recipes (78 lines)
- `.github/skills/design-system/reference/icons.md` — Icons: a 3-tier system, never emoji (43 lines)
- `.github/skills/design-system/reference/rules.md` — Hard rules: locks and bans (31 lines)
- `.github/skills/design-system/reference/typography.md` — Choosing a distinctive type pairing (24 lines)
- `.github/skills/design-system/templates/CLAUDE.md` — Design System: {{BRAND}} · Brand rules for generating UI (36 lines)
- `.github/skills/design-system/templates/storybook.html` — HTML file: 65 lines
- `.github/skills/design-system/templates/tailwind.config.js` — JavaScript module: 1 exports, 0 functions, 50 lines
- `.github/skills/design-system/templates/tokens.css` — ========================================================================== (157 lines)
- `.github/skills/fluid/SKILL.md` — - (72 lines)
- `.github/skills/fluid/components/AmbientField.tsx` — React component: DEFAULT_BLOBS, AmbientField; hooks: useRef, useEffect
- `.github/skills/fluid/components/Marquee.tsx` — React component: Marquee
- `.github/skills/fluid/components/Odometer.tsx` — Test suite: 1 test blocks, test
- `.github/skills/fluid/components/Reveal.tsx` — React component: Reveal
- `.github/skills/fluid/components/ScrollContext.tsx` — React component: ScrollProvider; hooks: useContext
- `.github/skills/fluid/components/ScrollProgress.tsx` — React component: ScrollProgress
- `.github/skills/fluid/components/SmoothScroll.tsx` — React component: SmoothScroll; hooks: useState, useEffect
- `.github/skills/fluid/components/marquee.css` — CSS file: 28 lines
- `.github/skills/fluid/detector/README.md` — fluid detector (24 lines)
- `.github/skills/fluid/detector/core.mjs` — fluid detector core: the importable scan engine behind detect.mjs and the fluid-skills ... (116 lines)
- `.github/skills/fluid/detector/deep.mjs` — fluid detector, Tier 3 (--deep): DOM-aware checks that the regex core cannot see. Runs ... (195 lines)
- `.github/skills/fluid/detector/detect.mjs` — fluid detector: a deterministic scanner for motion and generic-look anti-patterns. Run ... (32 lines)
- `.github/skills/fluid/detector/rules.mjs` — Deterministic anti-pattern rules for the `fluid` detector. No API calls. Each rule: { i... (204 lines)
- `.github/skills/fluid/reference/anti-patterns.md` — Anti-patterns checklist (56 lines)
- `.github/skills/fluid/reference/easings-and-durations.md` — Easings and durations (42 lines)
- `.github/skills/fluid/reference/principles.md` — Principles, condensed to rules (33 lines)
- `.github/skills/output/SKILL.md` — - (21 lines)
- `.github/skills/redesign/SKILL.md` — - (23 lines)
- `.github/skills/refine/SKILL.md` — - (57 lines)
- `.github/skills/refine/reference/commands.md` — Refine commands (121 lines)
- `.github/workflows/ci.yml` — GitHub Actions workflow
- `.github/workflows/ingest_knowledge.yml` — GitHub Actions workflow
- `.gitignore` — ============================================================================== Git Igno... (42 lines)
- `.npmignore` — Development files & scripts (22 lines)
- `AGENTS.md` — ZeroGravity OS (AGENTS.md) (7 lines)
- `CONTRIBUTING.md` — Contributing to Antigravity Supercoder OS (60 lines)
- `LICENSE` — Text file: 17 lines
- `Makefile` — ============================================================================== 🌌 ZeroGr... (64 lines)
- `README.md` — 🌌 ZeroGravity OS ## *Autonomous Supercoder OS • Zero-Default MCPs • 28 Workflows* (155 lines)
- `bin/cli.js` — functions: main, printBanner, parseArgs, copyDirSync
- `install_agy_staff.py` — Python module (0 functions, 0 classes)
- `openhands_config.toml` — TOML config: core, agent, llm
- `pack.sh` — Shell script (env); functions: print_banner, usage, check_cmd
- `package.json` — package.json
- `tests/test_all.py` — Tests for ScriptImportability, CLIHelpExits, JSONDataSchemas: 13 test functions/methods
- `unpack-plugins.sh` — Shell script (env)

