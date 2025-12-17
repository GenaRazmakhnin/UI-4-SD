# FHIR Profile Builder: Research Report & Implementation Plan 2025

## Executive Summary

This document defines a **2025-focused research report and implementation plan** for building a **UI-first FHIR Profile Builder**: a low-cognitive-load authoring experience backed by a deterministic, testable compiler pipeline. The solution is designed to reuse our Rust FSH toolchain **maki** (parser/semantic model/build/lint/format/GoFSH) while introducing an **IR-first** representation that supports **round-trip** between:

- **StructureDefinition** (authoritative artifact for IG Publisher/validators)
- **FSH** (optional import/export path; compatible with SUSHI where feasible)

### 2025 Product Statement

- **Primary user**: profiling/IG teams who need safe and fast authoring for complex constraints (slicing, extensions, terminology, invariants).
- **Primary value**: best-in-class **authoring UX** (low cognitive load) plus deterministic outputs, predictable diffs, and guardrails that prevent invalid/fragile profiles.
- **Primary deployment**: local-first (Rust backend + embedded React UI), with a path to hosted deployments later.
- **Primary scope**: Full **Implementation Guide (IG) authoring** — multiple profiles, extensions, value sets, and instances with correct cross-reference resolution.

### Principles (Non-Negotiables)

- **Determinism**: identical inputs → byte-identical exported artifacts (canonical JSON + stable ordering).
- **Round-trip safety**: imports must not silently drop information; preserve unknown/unmodeled fields.
- **Validation parity**: converge on IG Publisher / HL7 Validator behavior via layered validation + parity regression tests.
- **Incremental delivery**: MVP is narrow and reliable; advanced profiling features land behind explicit milestones.

### 2025 Non-Goals (MVP)

- Real-time multi-user collaboration (CRDT/OT) in the first release.
- A brand-new public DSL intended to replace FSH; IR is internal with import/export paths.
- Perfect textual round-tripping of arbitrary SD ↔ FSH (we target semantic equivalence, not formatting equivalence).
- Replacing the broader ecosystem toolchain: we integrate with **existing** validation/publishing workflows (maki + IG Publisher/validators) rather than inventing a new publishing tool.

### Toolchain Alignment (Important)

`maki` uses Rust **edition 2024**; this repo must align its Rust toolchain/MSRV accordingly (tracked as an explicit setup milestone).

---

## 1. Capability Comparison Table

### 1.1 Tooling Landscape (2025) — Key Observations

- **Forge** remains the most mature interactive profiling UX, but is tied to commercial workflows and can be difficult to standardize in Git-first pipelines.
- **SUSHI/FSH** remains the most automation-friendly “source format”, but it pushes complexity into text (especially slicing/terminology/invariants) and increases onboarding cost for non-experts.
- **Lightweight GUIs** often handle the “easy 80%” but struggle with edge cases (slicing discriminator rules, extension context rules, terminology binding validation, invariants) and with deterministic export behavior.
- **The arbiter is still Java tooling**: IG Publisher and HL7 Validator define downstream acceptance, so parity testing against them is mandatory for credibility.

### Current Tools vs Target Solution

| Capability | Firely Forge | Kodjin Profiler | SUSHI/FSH | SMART FRED | **Our Target** |
|------------|--------------|-----------------|-----------|------------|----------------|
| **Deployment** | Desktop (Windows/Mac) | VS Code Plugin | CLI | Web | **Web App** |
| **Pricing** | Commercial (Simplifier subscription) | Free | Free | Free | **Free/Open** |
| **FHIR Versions** | R4, R4B, R5 | R4 | R4, R4B, R5 | DSTU2, STU3 | **R4, R4B, R5** |
| **Authoring UX** | GUI forms | JSON editing | Text/DSL | JSON forms | **Visual + DSL** |
| **Slicing UI** | Good | Limited | Text rules | None | **Visual wizard** |
| **Extension Builder** | Good | Basic | Text rules | None | **Visual + search** |
| **Terminology Binding** | Good | Manual | Text rules | Manual | **Integrated picker** |
| **Constraint/Invariant Editor** | FHIRPath editor | Manual | Text | None | **Visual + FHIRPath** |
| **Validation** | Real-time | Server-side | Basic | None | **Real-time** |
| **Package Management** | Simplifier integration | Manual | sushi-config | None | **Integrated browser** |
| **Dependency Search** | Limited | None | None | None | **Full-text search** |
| **FSH Import/Export** | None | None | Native | None | **Full round-trip** |
| **SD Import/Export** | Native | Native | GoFSH | Native | **Full round-trip** |
| **Collaboration** | Simplifier projects | Git | Git | None | **Future: real-time** |
| **Offline Support** | Yes | Yes | Yes | Yes | **Yes (local server)** |
| **Deterministic Output** | Unknown | Unknown | Yes | N/A | **Guaranteed** |
| **Undo/Redo** | Yes | Limited | N/A | Limited | **Full history** |

### 1.2 Import/Export Reality Check (What “Round-Trip” Means)

To keep guarantees implementable, we define three fidelity levels:

1. **Semantic fidelity (required)**: exported SD/FSH validates/compiles equivalently (same constraints) even if formatting/order differs.
2. **Lossless preservation (required for SD)**: unknown/unmodeled JSON fields survive import → edit → export (no silent drops).
3. **Textual fidelity (nice-to-have)**: preserve original FSH formatting/comments on round-trip; not required for MVP.

### Pain Points Addressed

| Problem | Current State | Our Solution |
|---------|---------------|--------------|
| Editing raw StructureDefinitions | Error-prone, verbose | Visual tree editor with guardrails |
| FSH learning curve | Requires DSL knowledge | Optional: use GUI or FSH |
| Forge requires Simplifier | Vendor lock-in, cost | Free, open-source |
| No dependency search | Manual URL hunting | Integrated package browser with search |
| Slicing complexity | Confusing in all tools | Step-by-step wizard with suggestions |
| Extension discovery | Manual, fragmented | Search across all loaded packages |
| Terminology binding | Copy-paste URLs | Visual picker with validation |
| Non-deterministic output | Causes diff noise | Guaranteed deterministic serialization |

---

## 2. Recommended Approach: IR-First Architecture

### Decision: IR-First with FSH/SD as Import/Export Formats

```
                    ┌─────────────────────┐
                    │   UI Interactions   │
                    └──────────┬──────────┘
                               │
                               ▼
┌──────────────┐    ┌─────────────────────┐    ┌──────────────┐
│     FSH      │───▶│   ProfiledResource  │◀───│ StructureDef │
│   (Import)   │    │       (IR)          │    │   (Import)   │
└──────────────┘    └──────────┬──────────┘    └──────────────┘
                               │
                    ┌──────────┴──────────┐
                    ▼                      ▼
             ┌──────────────┐      ┌──────────────┐
             │     FSH      │      │ StructureDef │
             │   (Export)   │      │   (Export)   │
             └──────────────┘      └──────────────┘
```

### Rationale

| Approach | Pros | Cons | Verdict |
|----------|------|------|---------|
| **SD-First** | Direct FHIR compatibility | Complex to edit, poor UX for slicing/extensions | Not recommended |
| **FSH-First** | Existing tooling, familiar to FSH users | Inherits FSH complexity, limited UI control | Fallback option |
| **IR-First** | Full control over UX, deterministic, clean architecture | New abstraction to maintain | **Recommended** |

**Why IR-First:**
1. **UI Flexibility**: Can represent partial/draft states that FSH/SD cannot
2. **Deterministic**: Same IR always produces identical output
3. **Undo/Redo**: Native support for operation tracking
4. **Validation Control**: Validate incrementally without full compilation
5. **Round-Trip Fidelity**: Track changes separately from inherited values
6. **FSH Compatibility**: Can import/export FSH for developer workflows

### 2.1 Source-of-Truth Policy (Avoiding “Two Sources of Truth”)

- **IR is the authoritative working state** during editing (including drafts/partial states).
- **StructureDefinition is the authoritative published artifact** (what IG Publisher consumes).
- **FSH is an interoperability path**:
  - Import: parse/analyze with `maki-core` into semantic resources, then map into IR.
  - Export: either emit SD then decompile to FSH (via `maki-decompiler`), or add a direct IR→FSH emitter later for higher quality.

### 2.2 Determinism Contract

- Exports must be stable across machines and runs:
  - canonical JSON serialization
  - stable ordering of arrays where FHIR does not define meaning by order (and preserving order where it does)
  - stable internal IDs for UI operations (never leaked into exported artifacts)

---

## 3. System Architecture

### High-Level Architecture: Rust Backend with Embedded UI

**Key Decision**: Native Rust HTTP backend serving embedded React UI (no WASM complexity)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              BROWSER                                         │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                    React Frontend (Embedded Static)                    │  │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐     │  │
│  │  │ Element     │ │ Constraint  │ │ Slicing     │ │ Package     │     │  │
│  │  │ Tree        │ │ Editor      │ │ Wizard      │ │ Browser     │     │  │
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘     │  │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐     │  │
│  │  │ Extension   │ │ Terminology │ │ Preview     │ │ Diagnostics │     │  │
│  │  │ Picker      │ │ Picker      │ │ Panel       │ │ Panel       │     │  │
│  │  └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘     │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                    │                                         │
│                              HTTP/JSON API                                   │
│                                    │                                         │
└────────────────────────────────────┼────────────────────────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         RUST BACKEND (server crate)                          │
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                         HTTP Server (Axum)                              │ │
│  │  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐          │ │
│  │  │ Static Assets   │ │ Profile API     │ │ Package API     │          │ │
│  │  │ (embedded UI)   │ │ /api/profiles/* │ │ /api/packages/* │          │ │
│  │  └─────────────────┘ └─────────────────┘ └─────────────────┘          │ │
│  │  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐          │ │
│  │  │ Validation API  │ │ Search API      │ │ Export API      │          │ │
│  │  │ /api/validate   │ │ /api/search     │ │ /api/export/*   │          │ │
│  │  └─────────────────┘ └─────────────────┘ └─────────────────┘          │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                    │                                         │
│                                    ▼                                         │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                    Profile Builder Engine                               │ │
│  │  ┌──────────────────────────────────────────────────────────────────┐  │ │
│  │  │ ProfileBuilderEngine                                              │  │ │
│  │  │  ├── Document Manager (open, save, close profiles)                │  │ │
│  │  │  ├── IR Operations (create, update, delete elements)              │  │ │
│  │  │  ├── Import (SD → IR, FSH → IR via maki-core)                     │  │ │
│  │  │  ├── Export (IR → SD, IR → FSH deterministic)                     │  │ │
│  │  │  ├── Validation (incremental + full)                              │  │ │
│  │  │  └── Undo/Redo (edit history per document)                        │  │ │
│  │  └──────────────────────────────────────────────────────────────────┘  │ │
│  │                                 │                                        │ │
│  │                                 ▼                                        │ │
│  │  ┌──────────────────────────────────────────────────────────────────┐  │ │
│  │  │ maki-core Integration                                             │  │ │
│  │  │  ├── FSH Parser (Rowan CST)                                       │  │ │
│  │  │  ├── Semantic Analyzer                                            │  │ │
│  │  │  ├── Fishing Context (resource resolution)                        │  │ │
│  │  │  ├── Profile Exporter (SD generation)                             │  │ │
│  │  │  ├── Decompiler (GoFSH - FHIR to FSH)                             │  │ │
│  │  │  └── Rule Engine (linting, validation)                            │  │ │
│  │  └──────────────────────────────────────────────────────────────────┘  │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                    │                                         │
│                                    ▼                                         │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                         Storage Layer                                   │ │
│  │  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐          │ │
│  │  │ SQLite          │ │ File System     │ │ Package Cache   │          │ │
│  │  │ (CanonicalFacade│ │ (projects,      │ │ (~/.maki/       │          │ │
│  │  │  from maki)     │ │  exports)       │ │  packages/)     │          │ │
│  │  └─────────────────┘ └─────────────────┘ └─────────────────┘          │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                        EXTERNAL SERVICES                                     │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐               │
│  │ FHIR Package    │ │ Terminology     │ │ FHIR Servers    │               │
│  │ Registry        │ │ Services        │ │ (optional)      │               │
│  │ (packages.fhir) │ │ (tx.fhir.org)   │ │                 │               │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘               │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Why Rust Backend (Not WASM)

| Aspect | WASM Approach | Rust Backend Approach |
|--------|---------------|----------------------|
| **Complexity** | Complex WASM builds, memory management | Standard Rust binary |
| **Package Management** | IndexedDB workarounds | Direct SQLite (maki's CanonicalFacade) |
| **File Access** | Limited, requires uploads | Full filesystem access |
| **Performance** | Bundle size concerns, startup time | Native speed, no cold start |
| **Debugging** | Difficult | Standard Rust tooling |
| **Deployment** | Any static host | Single binary (can embed UI) |
| **Offline** | Yes (but complex) | Yes (local server) |

### Deployment Options

1. **Local Development Server** (default)
   ```bash
   cargo run -p server --port 3000
   # Opens browser to http://localhost:3000
   ```

2. **Single Binary with Embedded UI**
   ```bash
   # UI assets embedded via rust-embed
   ./profile-builder-ui  # Starts server and opens browser
   ```

3. **Docker Container**
   ```dockerfile
   FROM scratch
   COPY server /
   EXPOSE 3000
   CMD ["/server"]
   ```

### Module Breakdown

#### Rust Crates

| Crate | Purpose | Key Dependencies |
|-------|---------|------------------|
| `maki-core` | Existing FSH parser, semantic analyzer, exporters (via local path) | rowan, chumsky |
| `profile-builder` | **NEW**: IR types, operations, import/export | maki-core |
| `server` | **NEW**: HTTP API + embedded UI | axum, tokio, tower, rust-embed |

#### API Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/` | GET | Serve embedded React UI |
| `/api/profiles` | GET | List open profiles |
| `/api/profiles` | POST | Create new profile (from base) |
| `/api/profiles/:id` | GET | Get profile IR state |
| `/api/profiles/:id` | PATCH | Update element constraints |
| `/api/profiles/:id/import` | POST | Import SD or FSH |
| `/api/profiles/:id/export/sd` | GET | Export StructureDefinition |
| `/api/profiles/:id/export/fsh` | GET | Export FSH |
| `/api/profiles/:id/validate` | POST | Run validation |
| `/api/profiles/:id/undo` | POST | Undo last operation |
| `/api/profiles/:id/redo` | POST | Redo operation |
| `/api/packages` | GET | List installed packages |
| `/api/packages` | POST | Install package from registry |
| `/api/packages/search` | GET | Search package registry |
| `/api/search/resources` | GET | Search resources in loaded packages |
| `/api/search/extensions` | GET | Search available extensions |
| `/api/search/valuesets` | GET | Search value sets |

### 3.1 Integration with `maki` (Reuse vs Add)

**Reuse from maki:**
- FSH parsing + CST/AST + diagnostics
- Semantic analysis and package resolution (`CanonicalFacade`, `FishingContext`)
- Build parity infrastructure (SUSHI-compatible compilation + existing parity tests)
- Formatter + lint/auto-fix (useful for embedded “FSH power user” mode)
- Decompiler (FHIR → FSH) for the initial IR→FSH export strategy

**Add in UI-4-SD:**
- UI-oriented IR (draft states, stable node IDs, undo/redo operations)
- Lossless SD import/export strategy (preserve unknown fields)
- Validation orchestration + parity reporting, semantic diff, preview tooling

### 3.2 Validation Pipeline (Layered, Parity-Driven)

Validation must be fast enough for interactive editing, but must converge on “publisher truth”:

1. **IR validation (instant)**: structural invariants (min/max sanity, slice name uniqueness, type constraint shape).
2. **Terminology checks (cached, async)**: binding strength checks; code membership when a terminology service is available; degrade gracefully when offline.
3. **Publisher parity checks (on-demand / CI)**:
   - run HL7 Validator / IG Publisher against exported artifacts
   - store parity reports and regress them (no new false negatives; reduced false positives over time)

### 3.3 Storage, Versioning, Collaboration (Start Simple)

- **Local-first project format**: store IR + imports + exports + parity reports together; design for Git from day one.
- **Deterministic outputs** enable PR review (diff noise stays low) and safe branch/merge workflows.
- **Future real-time collaboration** can be added later as an event log + conflict resolution layer without changing IR semantics.

### 3.4 Developer Tooling (Diff, Preview, Packaging)

- **Semantic diff**: diff IR operations/constraints (not just raw JSON) for review and audit trails.
- **Formatting/linting**: reuse `maki` for FSH formatting/lint in “power user” workflows.
- **Preview**: instant SD/FSH preview plus a “publishability” checklist (required metadata, canonical, dependencies).
- **Packaging**: generate IG-ready outputs (FHIR JSON, `package.tgz`, `ImplementationGuide`/menu scaffolding) by reusing maki’s export/IG generation where possible.

---

## 4. UX Proposal

### 4.1 Key Screens / Workflows

- **Project browser**: open/create project, manage FHIR package dependencies, choose FHIR release target (R4/R4B/R5).
- **Artifact explorer**: profiles/extensions/value sets/code systems/instances with search + filtering by package.
- **Element tree + inspector**: virtualized tree with inherited vs modified highlighting; inspector applies explicit operations (cardinality, flags, types, bindings, slicing).
- **Slicing wizard**: discriminator selection, rules (open/closed), slice patterns, and a clear “what changes in SD” preview.
- **Extension picker**: search across loaded packages; show context rules and compatibility hints.
- **Terminology picker**: browse ValueSets, expansions, and binding strengths; cache-aware and offline-tolerant.
- **Constraint editor**: FHIRPath editor with templates and diagnostics mapped back to element paths.
- **Preview + diff**: SD/FSH preview plus a semantic diff (“what constraints changed”) for review.
- **Validation panel**: fast incremental checks + optional “publisher parity run” with a report.

### 4.2 UX Design Goals (Low Cognitive Load)

- No hidden side effects: every edit is visible, undoable, and explains what will be exported.
- Stable UI: selections do not jump; large profiles remain responsive via virtualization and incremental re-render.
- Progressive disclosure: advanced tools (slicing/invariants/terminology) expand only when relevant.

### 4.3 “No New Authoring Tooling” Constraint (How We Still Win on UX)

We do **not** require teams to adopt a new authoring language/toolchain to get a good experience:

- UI edits operate on the internal IR and export **StructureDefinition** as the primary artifact.
- For power users, FSH remains available as an **interop mode** (import/export) via `maki`, not as a new mandatory source format.
- Validation and publishability are measured against **existing** ecosystem truth (IG Publisher / HL7 Validator), with parity regression tests.

---

## 5. Project & IG-Level Architecture

### 5.1 Multi-File IG Support

The Profile Builder operates at the **project/IG level**, not just individual profiles. A project contains multiple inter-related artifacts that must be validated together.

#### Project Structure (On Disk)

```
my-ig/
├── sushi-config.yaml           # IG metadata, dependencies, FHIR version
├── input/
│   ├── fsh/
│   │   ├── profiles/
│   │   │   ├── MyPatient.fsh
│   │   │   └── MyObservation.fsh
│   │   ├── extensions/
│   │   │   └── MyExtension.fsh
│   │   ├── valuesets/
│   │   │   └── MyValueSet.fsh
│   │   └── instances/
│   │       └── ExamplePatient.fsh
│   └── resources/              # Pre-built JSON resources
│       └── StructureDefinition-*.json
├── output/                     # Generated artifacts (IG Publisher input)
└── .profile-builder/           # Local cache and state
    ├── ir-cache/               # Serialized IR for quick reload
    └── validation-cache/       # Cached validation results
```

#### Project Model (IR)

```
Project
├── id: UUID
├── name: String
├── fhir_version: R4 | R4B | R5
├── canonical_base: String (e.g., "http://example.org/fhir")
├── dependencies: Vec<PackageDependency>
├── resources: HashMap<CanonicalUrl, ProjectResource>
├── dirty_resources: HashSet<CanonicalUrl>  # Unsaved changes
└── validation_state: ProjectValidationState

ProjectResource
├── canonical_url: CanonicalUrl
├── kind: Profile | Extension | ValueSet | CodeSystem | Instance | Other
├── source: FSH | JSON | IR  # Where this resource came from
├── ir: Option<ProfiledResource>  # Loaded IR (lazy)
├── file_path: PathBuf
└── dependencies: Vec<CanonicalUrl>  # What this resource references

PackageDependency
├── package_id: String (e.g., "hl7.fhir.us.core")
├── version: String
├── resolved: bool
└── resources: HashMap<CanonicalUrl, CachedResource>
```

### 5.2 Cross-Reference Resolution

References between resources (e.g., a profile referencing an extension, or an instance referencing a profile) must resolve correctly:

```rust
impl Project {
    /// Resolve a canonical URL to a resource, searching:
    /// 1. Project-local resources (highest priority)
    /// 2. Loaded package dependencies
    /// 3. Core FHIR spec resources
    fn resolve(&self, url: &CanonicalUrl) -> Option<&ResolvedResource> {
        self.resources.get(url)
            .or_else(|| self.search_dependencies(url))
            .or_else(|| self.core_spec.get(url))
    }

    /// Find all resources that reference the given URL
    fn find_dependents(&self, url: &CanonicalUrl) -> Vec<&ProjectResource> {
        self.resources.values()
            .filter(|r| r.dependencies.contains(url))
            .collect()
    }

    /// Validate all cross-references in the project
    fn validate_references(&self) -> Vec<Diagnostic> {
        self.resources.values()
            .flat_map(|r| r.dependencies.iter()
                .filter(|dep| self.resolve(dep).is_none())
                .map(|dep| Diagnostic::unresolved_reference(r, dep)))
            .collect()
    }
}
```

### 5.3 Incremental Compilation

When a resource changes, only affected resources need revalidation:

```
Change to MyExtension.fsh
        │
        ▼
┌───────────────────────┐
│ Invalidate IR cache   │
│ for MyExtension       │
└───────────┬───────────┘
            │
            ▼
┌───────────────────────┐
│ Find dependents:      │
│ - MyPatient (uses it) │
│ - ExamplePatient      │
└───────────┬───────────┘
            │
            ▼
┌───────────────────────┐
│ Mark for revalidation │
│ (lazy, on-demand)     │
└───────────────────────┘
```

---

## 6. Quick Constraints Panel

### 6.1 Overview

Many profiling operations are repetitive. The **Quick Constraints Panel** provides one-click access to common operations via:

1. **Context menu** (right-click on element in tree)
2. **Toolbar** (always visible when element selected)
3. **Keyboard shortcuts** (power users)

### 6.2 Quick Actions

| Action | Shortcut | Description | API Operation |
|--------|----------|-------------|---------------|
| **Make Required** | `Ctrl+R` | Set `min: 1` | `PATCH /profiles/:id/elements/:path { min: 1 }` |
| **Prohibit** | `Ctrl+0` | Set `max: 0` | `PATCH /profiles/:id/elements/:path { max: "0" }` |
| **Make Single** | `Ctrl+1` | Set `max: 1` | `PATCH /profiles/:id/elements/:path { max: "1" }` |
| **Toggle Must Support** | `Ctrl+M` | Toggle `mustSupport` flag | `PATCH /profiles/:id/elements/:path { mustSupport: toggle }` |
| **Add Extension** | `Ctrl+E` | Open extension picker for this element | Opens modal with filtered extensions |
| **Add Slice** | `Ctrl+S` | Start slicing wizard for this element | Opens SliceWizard modal |
| **Bind to ValueSet** | `Ctrl+B` | Open terminology picker | Opens ValueSetPicker modal |
| **Reset to Base** | `Ctrl+Shift+R` | Remove all constraints, inherit from base | `DELETE /profiles/:id/elements/:path/constraints` |

### 6.3 Context Menu Design

```
┌─────────────────────────────────┐
│ Patient.identifier              │
├─────────────────────────────────┤
│ ▸ Cardinality                   │
│   ├─ Make Required (1..*)       │
│   ├─ Make Single (0..1)         │
│   ├─ Prohibit (0..0)            │
│   └─ Custom...                  │
├─────────────────────────────────┤
│ ☐ Must Support                  │
│ ☐ Is Modifier                   │
├─────────────────────────────────┤
│ ▸ Binding                       │
│   ├─ Recent: USCoreRace         │
│   ├─ Recent: AdministrativeGender│
│   ├─ Favorites ▸                │
│   └─ Browse ValueSets...        │
├─────────────────────────────────┤
│ Add Extension...                │
│ Add Slice...                    │
│ Add Constraint (FHIRPath)...    │
├─────────────────────────────────┤
│ Reset to Base                   │
│ Copy Path                       │
│ View in Base Definition         │
└─────────────────────────────────┘
```

### 6.4 Favorites & Recent Bindings

Track frequently used value sets and extensions per-user:

```typescript
// stores/favorites/model.ts
interface Favorites {
  valueSets: CanonicalUrl[];      // User's favorite value sets
  extensions: CanonicalUrl[];     // User's favorite extensions
  recentBindings: BindingRecord[]; // Last 10 bindings applied
  recentExtensions: CanonicalUrl[]; // Last 10 extensions added
}

// Persisted to localStorage or backend user settings
export const $favorites = createStore<Favorites>(loadFromStorage());
```

### 6.5 Toolbar Component

```tsx
// components/profile/QuickActionsToolbar/QuickActionsToolbar.tsx
export function QuickActionsToolbar() {
  const element = useUnit($selectedElement);
  const updateConstraint = useUnit(constraintUpdated);

  if (!element) return null;

  return (
    <Group className={styles.toolbar}>
      <Tooltip label="Make Required (Ctrl+R)">
        <ActionIcon onClick={() => updateConstraint({ min: 1 })}>
          <IconRequired />
        </ActionIcon>
      </Tooltip>
      <Tooltip label="Prohibit (Ctrl+0)">
        <ActionIcon onClick={() => updateConstraint({ max: "0" })}>
          <IconProhibit />
        </ActionIcon>
      </Tooltip>
      <Tooltip label="Toggle Must Support (Ctrl+M)">
        <ActionIcon
          variant={element.mustSupport ? "filled" : "subtle"}
          onClick={() => updateConstraint({ mustSupport: !element.mustSupport })}
        >
          <IconMustSupport />
        </ActionIcon>
      </Tooltip>
      <Divider orientation="vertical" />
      <Button leftSection={<IconExtension />} variant="subtle" onClick={openExtensionPicker}>
        Add Extension
      </Button>
      <Button leftSection={<IconSlice />} variant="subtle" onClick={openSliceWizard}>
        Add Slice
      </Button>
      <Button leftSection={<IconValueSet />} variant="subtle" onClick={openBindingPicker}>
        Bind ValueSet
      </Button>
    </Group>
  );
}
```

---

## 7. Project Templates & Onboarding

### 7.1 Template System

New projects start from templates that provide:
- Pre-configured dependencies
- Base profiles/extensions as starting points
- Example instances for validation
- sushi-config.yaml with sensible defaults

### 7.2 Built-in Templates

| Template | Description | Dependencies | Includes |
|----------|-------------|--------------|----------|
| **Blank R4** | Empty R4 project | `hl7.fhir.r4.core` | Empty sushi-config |
| **Blank R4B** | Empty R4B project | `hl7.fhir.r4b.core` | Empty sushi-config |
| **Blank R5** | Empty R5 project | `hl7.fhir.r5.core` | Empty sushi-config |
| **US Core R4** | US Core starter | `hl7.fhir.us.core@6.1.0` | USCorePatient, USCorePractitioner stubs |
| **International Patient Access** | IPA starter | `hl7.fhir.uv.ipa@1.0.0` | IPAPatient, IPAObservation stubs |
| **mCODE** | Oncology starter | `hl7.fhir.us.mcode@3.0.0` | CancerPatient stub |
| **AU Core** | Australian Core | `hl7.fhir.au.core@1.0.0` | AUCorePatient stub |
| **Custom URL** | Import from package | User-specified | Package contents |

### 7.3 Template Definition (YAML)

```yaml
# templates/us-core-r4.yaml
name: "US Core R4 Starter"
description: "Start a new IG based on US Core R4"
fhir_version: "4.0.1"
canonical_base: "http://example.org/fhir"

dependencies:
  - id: hl7.fhir.us.core
    version: "6.1.0"
  - id: us.nlm.vsac
    version: "0.18.0"

starter_profiles:
  - name: MyPatient
    parent: http://hl7.org/fhir/us/core/StructureDefinition/us-core-patient
    description: "Patient profile for my IG"

  - name: MyObservation
    parent: http://hl7.org/fhir/us/core/StructureDefinition/us-core-observation-clinical-result
    description: "Observation profile for my IG"

starter_extensions: []

starter_valuesets: []

example_instances:
  - name: ExamplePatient
    profile: MyPatient
    description: "Example patient instance"
```

### 7.4 New Project Wizard UI

```
┌─────────────────────────────────────────────────────────────────┐
│                    Create New Project                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Project Name: [my-implementation-guide        ]                │
│                                                                  │
│  Canonical URL: [http://example.org/fhir       ]                │
│                                                                  │
│  Location: [/Users/dev/projects/my-ig    ] [Browse]             │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│  Select Template:                                                │
│                                                                  │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐   │
│  │  ★ Recommended  │ │                 │ │                 │   │
│  │   US Core R4    │ │    Blank R4     │ │    Blank R5     │   │
│  │                 │ │                 │ │                 │   │
│  │  hl7.fhir.us.   │ │  Start fresh    │ │  Latest FHIR    │   │
│  │  core@6.1.0     │ │  with R4        │ │  specification  │   │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘   │
│                                                                  │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐   │
│  │   Int'l Patient │ │     mCODE       │ │   AU Core       │   │
│  │     Access      │ │                 │ │                 │   │
│  │                 │ │   Oncology      │ │  Australian     │   │
│  │  Universal      │ │   profiles      │ │  profiles       │   │
│  └─────────────────┘ └─────────────────┘ └─────────────────┘   │
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  Import from Package URL:                                │   │
│  │  [                                              ] [Load] │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
├─────────────────────────────────────────────────────────────────┤
│                              [Cancel]  [Create Project]          │
└─────────────────────────────────────────────────────────────────┘
```

### 7.5 API Endpoints for Projects

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/projects` | GET | List all projects |
| `/api/projects` | POST | Create new project (from template) |
| `/api/projects/:id` | GET | Get project metadata and resource list |
| `/api/projects/:id` | DELETE | Delete project |
| `/api/projects/:id/resources` | GET | List all resources in project |
| `/api/projects/:id/resources/:url` | GET | Get specific resource IR |
| `/api/projects/:id/resources/:url` | PUT | Update resource |
| `/api/projects/:id/resources` | POST | Create new resource |
| `/api/projects/:id/validate` | POST | Validate entire project |
| `/api/projects/:id/export` | POST | Export project (IG Publisher format) |
| `/api/templates` | GET | List available templates |

---

## 8. Data Model / IR Sketch (Profile-Level)

### Core IR Entities

```
ProfileDocument
├── metadata: DocumentMetadata
├── resources: Vec<ProfiledResource>
├── dependencies: DependencyGraph
└── change_tracker: ChangeTracker

ProfiledResource
├── id: UUID
├── kind: Profile | Extension | Logical | Resource
├── canonical_url: String
├── name: String
├── title: Option<String>
├── base_definition: CanonicalReference
├── elements: ElementTree
├── metadata: ResourceMetadata
└── change_tracker: ChangeTracker

ElementTree
├── root: ElementNode
├── index: HashMap<ElementPath, ElementId>
└── snapshot_cache: Option<Vec<ElementDefinition>>

ElementNode
├── id: ElementId
├── path: ElementPath (e.g., "Patient.identifier")
├── base_element: Option<ElementDefinition>
├── constraints: ElementConstraints
├── children: Vec<ElementNode>
├── slicing: Option<SlicingDefinition>
├── slices: Vec<SliceNode>
└── change_state: Inherited | Modified | Added | Removed

ElementConstraints
├── min: Option<u32>
├── max: Option<Cardinality>
├── types: Option<Vec<TypeConstraint>>
├── fixed: Option<FixedValue>
├── pattern: Option<PatternValue>
├── binding: Option<BindingConstraint>
├── must_support: Option<bool>
├── is_modifier: Option<bool>
├── is_summary: Option<bool>
├── short: Option<String>
├── definition: Option<String>
├── invariants: Vec<InvariantConstraint>
└── mappings: Vec<MappingConstraint>

SlicingDefinition
├── discriminators: Vec<Discriminator>
├── description: Option<String>
├── ordered: bool
└── rules: Open | Closed | OpenAtEnd

SliceNode
├── name: String
├── element: ElementNode
└── condition: SliceCondition

ChangeTracker
├── baseline: HashMap<ElementId, ElementSnapshot>
├── version: u64
└── history: EditHistory (undo/redo stack)
```

### 5.1 Lossless SD Import/Export Strategy (No Silent Drops)

Because `StructureDefinition` has many optional/rare fields (and implementer-specific extensions), SD import must be **lossless**:

- On import, store the **original JSON** alongside a typed view for the fields we actively edit.
- Apply IR edits as a **patch layer** that updates only the touched fields/paths.
- On export, merge patches back into the preserved JSON, then canonicalize the output for determinism.

This approach lets us ship MVP editing features without claiming full semantic coverage of every SD field on day one.

### 5.2 Deterministic Serialization Rules

- Canonical JSON formatting (stable whitespace, newline, ordering) for exported artifacts.
- Stable ordering of:
  - `differential.element` by `path` + slice name rules
  - `snapshot.element` by canonical snapshot order
  - other arrays only when FHIR semantics do not rely on order (otherwise preserve imported order)

### Integration with Maki

The IR extends maki's existing `SemanticModel` and `FhirResource` types:

```rust
// Bridge between ProfileBuilder IR and maki
impl From<&maki::FhirResource> for ProfiledResource { ... }
impl From<&ProfiledResource> for maki::FhirResource { ... }

// Reuse maki's fishing context for dependency resolution
impl ProfileBuilderEngine {
    fn resolve_base(&self, url: &str) -> Result<StructureDefinition> {
        self.fishing_context.fish_structure_definition(url)
    }
}
```

---

## 9. Milestone Roadmap 2025

### Phase 1: MVP (8-10 weeks)

**Goal**: Basic profile viewing and editing with SD import/export

| Week | Deliverables |
|------|--------------|
| 1-2 | IR types in `profile-builder`, Axum server skeleton |
| 3-4 | SD import → IR, IR → SD export with differential generation |
| 5-6 | React app scaffold, element tree viewer |
| 7-8 | Cardinality editor, flags editor (MS/modifier/summary) |
| 9-10 | Basic validation (cardinality bounds, type refinement), diagnostics panel |

**MVP Features:**
- Import existing StructureDefinition JSON
- View element tree with inherited vs modified highlighting
- Edit cardinality (min/max)
- Edit flags (mustSupport, isModifier, isSummary)
- Edit short/definition text
- Export StructureDefinition JSON
- Basic validation with error display

### Phase 2: Beta (8-10 weeks)

**Goal**: Full profiling capabilities, package management, FSH support

| Week | Deliverables |
|------|--------------|
| 1-2 | Type constraint editor, binding editor |
| 3-4 | Slicing UI wizard, discriminator editor |
| 5-6 | Extension picker with package search |
| 7-8 | FSH import/export via maki integration |
| 9-10 | Package browser, dependency search |

**Beta Features:**
- Type refinement (restrict allowed types)
- Terminology binding with strength selection
- Visual slicing setup wizard
- Extension browser with search
- FSH import/export
- FHIR package browser and installation
- Search resources across loaded packages
- Undo/redo for all operations

### Phase 3: Production (6-8 weeks)

**Goal**: Production-ready with comprehensive validation and polish

| Week | Deliverables |
|------|--------------|
| 1-2 | Invariant/constraint editor with FHIRPath |
| 3-4 | Comprehensive validation rules, quick fixes |
| 5-6 | Performance optimization, large profile handling |
| 7-8 | Documentation, testing, error handling polish |

**Production Features:**
- FHIRPath constraint editor with syntax validation
- Full validation parity with IG Publisher
- Quick fix suggestions for common errors
- Optimized for profiles with 500+ elements
- Comprehensive error messages
- User documentation

### Phase 4: Advanced (Future)

**Goal**: Collaboration and ecosystem integration

- Real-time collaboration (multiple editors)
- Git integration for version control
- IG Publisher integration
- Simplifier.net sync (optional)
- VS Code extension (LSP)

---

## 10. Risk Register

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| **Complex slicing edge cases** | Incorrect SD output | High | Comprehensive test suite against SUSHI output |
| **Round-trip fidelity loss** | User data corruption | High | Preserve original structure, baseline tracking, diff tests |
| **Lossless SD merge bugs** | Silent field drops or corruption | Medium | Preserve raw JSON + apply minimal patches; golden tests for “unmodified export == canonicalized import” |
| **Publisher parity drift** | “Works in tool” but fails in IG Publisher | Medium | Add parity harness + regressions; treat parity failures as release blockers |
| **Extension context validation** | Invalid profiles accepted | Medium | Port maki's extension validation rules |
| **Large profile performance** | Slow UI (>500 elements) | Medium | Virtual scrolling, lazy loading, incremental updates |
| **FHIRPath expression validation** | Invalid invariants | Medium | Use existing FHIRPath parser, deferred to IG Publisher |
| **Concurrent API requests** | Race conditions, stale state | Medium | Optimistic locking, version tracking, request queuing |
| **Terminology service availability** | Cannot validate bindings | Low | Graceful degradation, offline code lists, caching |
| **maki API changes** | Integration breakage | Low | Version pinning, integration tests |
| **Rust toolchain mismatch** | Build failures / blocked onboarding | Low | Pin toolchain (`rust-toolchain.toml`) and document prerequisites |
| **Local server port conflicts** | Can't start server | Low | Auto port selection, configurable port |
| **Cross-platform binary distribution** | Complex release process | Low | Cross-compilation CI, static linking |

### Critical Mitigations

**Round-Trip Fidelity Testing:**
```bash
# Run against US Core, IPA, mCODE profiles
cargo test --package profile-builder roundtrip_tests
```

**Determinism Testing:**
```bash
# Same IR must produce byte-identical output
cargo test --package profile-builder determinism_tests
```

**SUSHI Comparison Testing:**
```bash
# Compare output against SUSHI for complex FSH
npm run test:sushi-comparison
```

---

## 11. Acceptance Criteria

### Functional Requirements

| Requirement | Acceptance Criteria |
|-------------|---------------------|
| **SD Import** | Import R4/R4B/R5 StructureDefinition while preserving unknown fields (no silent drops) |
| **SD Export** | Export produces valid StructureDefinition that passes IG Publisher |
| **FSH Import** | Import FSH files, produce identical SD to SUSHI (semantic equivalence) |
| **FSH Export** | Export FSH that SUSHI compiles to equivalent SD |
| **Determinism** | Same IR state produces byte-identical JSON output |
| **Validation** | Catch all errors that IG Publisher catches (no false negatives) |
| **Undo/Redo** | Unlimited undo/redo with full state restoration |

### Non-Functional Requirements

| Requirement | Target | Measurement |
|-------------|--------|-------------|
| **Initial Load** | < 3s on 3G | Lighthouse performance score |
| **Edit Latency** | < 100ms | Time from keystroke to UI update |
| **Validation Latency** | < 500ms | Time from edit to diagnostics display |
| **Export Latency** | < 1s for 500 elements | Time to generate SD JSON |
| **Memory Usage** | < 200MB for large profile | Chrome DevTools memory profiler |
| **Package Install** | < 10s for us-core | Time from click to indexed |
| **Search Results** | < 200ms | Time from query to results |

### UX Acceptance Criteria (Must-Have)

| Workflow | Acceptance Criteria |
|----------|---------------------|
| **Create Profile** | Create a profile from a base resource and export a valid SD in < 5 minutes (first-time user, guided UI) |
| **Edit Cardinality** | Change min/max with immediate preview + undo in < 15 seconds, with clear impact explanation |
| **Add Extension** | Find and add an extension from loaded packages in < 60 seconds, with context compatibility warnings |
| **Create Slice** | Create a slice via the wizard with discriminator guidance and a deterministic SD diff preview |
| **Diagnose Error** | Validation errors link to the exact element and provide an actionable fix hint (where possible) |

### Quality Gates

**MVP Release:**
- [ ] All US Core R4 profiles import/export without errors
- [ ] Basic validation catches cardinality violations
- [ ] No critical bugs in element tree navigation
- [ ] Documentation for basic workflows

**Beta Release:**
- [ ] All IPA profiles import/export correctly
- [ ] Slicing works for identifier, extension, and coding patterns
- [ ] FSH round-trip passes for 95% of test cases
- [ ] Package browser installs packages from registry

**Production Release:**
- [ ] Validation parity with IG Publisher (no false negatives)
- [ ] Performance targets met for profiles with 500+ elements
- [ ] Zero data loss in round-trip testing (100+ profiles)
- [ ] Accessibility audit passed (WCAG 2.1 AA)

---

## 12. Project Structure (Monorepo)

```
UI-4-SD/
├── Cargo.toml                    # Rust workspace root
├── web/                          # (planned) React UI workspace
├── .cargo/
│   └── config.toml               # Cargo config (path to maki)
│
├── crates/
│   ├── profile-builder/          # Core library (IR, import/export)
│   │   ├── Cargo.toml            # depends on maki-core via path
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── ir/
│   │       │   ├── mod.rs
│   │       │   ├── document.rs   # ProfileDocument
│   │       │   ├── resource.rs   # ProfiledResource
│   │       │   ├── element.rs    # ElementNode, ElementTree
│   │       │   ├── constraint.rs # ElementConstraints
│   │       │   ├── slicing.rs    # SlicingDefinition, SliceNode
│   │       │   └── tracking.rs   # ChangeTracker, EditHistory
│   │       ├── import/
│   │       │   ├── mod.rs
│   │       │   ├── sd_import.rs  # StructureDefinition -> IR
│   │       │   └── fsh_import.rs # FSH -> IR (via maki-core)
│   │       ├── export/
│   │       │   ├── mod.rs
│   │       │   ├── sd_export.rs  # IR -> StructureDefinition
│   │       │   ├── fsh_export.rs # IR -> FSH
│   │       │   └── deterministic.rs
│   │       ├── validation/
│   │       │   ├── mod.rs
│   │       │   └── rules/
│   │       └── operations/
│   │           ├── mod.rs
│   │           ├── constraint_ops.rs
│   │           ├── slicing_ops.rs
│   │           └── extension_ops.rs
│   │
│   └── server/                   # HTTP server with embedded UI
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── server.rs         # Axum setup
│           ├── state.rs          # AppState
│           ├── routes/
│           │   ├── mod.rs
│           │   ├── profiles.rs
│           │   ├── packages.rs
│           │   ├── search.rs
│           │   └── static_files.rs
│           └── api/
│               ├── mod.rs
│               ├── types.rs      # DTOs
│               └── openapi.rs    # OpenAPI (utoipa)
│
└── web/                          # (planned) React frontend
    ├── package.json
    ├── tsconfig.json
    ├── vite.config.ts
    ├── src/
    │   └── ... (see detailed structure below)
    └── dist/                     # Vite build output (embedded in server)
```

### Cargo.toml (workspace root)

```toml
[workspace]
resolver = "2"
members = ["crates/*"]

[workspace.package]
# Align toolchain with `maki` (edition 2024)
edition = "2024"
rust-version = "1.85"

[workspace.dependencies]
# Use maki-core from local path
maki-core = { path = "../maki/crates/maki-core" }

# Common dependencies
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
axum = "0.7"
tower-http = { version = "0.5", features = ["cors", "fs"] }
uuid = { version = "1", features = ["v4", "serde"] }
thiserror = "1.0"
tracing = "0.1"
```

---

## 13. UI Application — Feature-Sliced Design (FSD) Architecture

### 13.1 Why FSD?

Feature-Sliced Design provides a scalable, maintainable architecture for complex applications:

- **Explicit dependencies**: layers can only import from layers below
- **Business-focused**: organized by domain features, not technical concerns
- **Scalable**: easy to add new features without touching existing code
- **Team-friendly**: clear boundaries for parallel development

### 13.2 FSD Layer Structure

```
┌─────────────────────────────────────────────────────────────────┐
│  app/          Application initialization, providers, routing   │
├─────────────────────────────────────────────────────────────────┤
│  pages/        Route compositions (combine widgets/features)    │
├─────────────────────────────────────────────────────────────────┤
│  widgets/      Complex UI blocks (sidebar, editor panels)       │
├─────────────────────────────────────────────────────────────────┤
│  features/     User interactions (edit-constraint, add-slice)   │
├─────────────────────────────────────────────────────────────────┤
│  entities/     Business entities (profile, element, package)    │
├─────────────────────────────────────────────────────────────────┤
│  shared/       Reusable infrastructure (ui, api, lib, config)   │
└─────────────────────────────────────────────────────────────────┘
         ▲ Dependencies flow DOWN only (no upward imports)
```

### 13.3 Directory Structure

```
web/
├── package.json
├── tsconfig.json
├── vite.config.ts
├── postcss.config.js
│
└── src/
    ├── index.tsx                    # Entry point
    │
    ├── app/                         # Layer: Application
    │   ├── index.tsx                # App component with providers
    │   ├── providers/
    │   │   ├── RouterProvider.tsx
    │   │   ├── EffectorProvider.tsx
    │   │   └── MantineProvider.tsx
    │   ├── routing/
    │   │   └── routes.tsx           # Route definitions
    │   └── styles/
    │       ├── global.css
    │       └── variables.css
    │
    ├── pages/                       # Layer: Pages (route compositions)
    │   ├── home/
    │   │   ├── ui/
    │   │   │   └── HomePage.tsx
    │   │   └── index.ts
    │   ├── project-editor/
    │   │   ├── ui/
    │   │   │   ├── ProjectEditorPage.tsx
    │   │   │   └── ProjectEditorPage.module.css
    │   │   └── index.ts
    │   ├── profile-editor/
    │   │   ├── ui/
    │   │   │   ├── ProfileEditorPage.tsx
    │   │   │   └── ProfileEditorPage.module.css
    │   │   └── index.ts
    │   └── packages/
    │       ├── ui/
    │       │   └── PackagesPage.tsx
    │       └── index.ts
    │
    ├── widgets/                     # Layer: Widgets (complex compositions)
    │   ├── app-shell/
    │   │   ├── ui/
    │   │   │   ├── AppShell.tsx
    │   │   │   ├── Sidebar.tsx
    │   │   │   └── Header.tsx
    │   │   └── index.ts
    │   ├── element-tree/
    │   │   ├── ui/
    │   │   │   ├── ElementTree.tsx
    │   │   │   ├── ElementNode.tsx
    │   │   │   └── ElementTree.module.css
    │   │   ├── model/               # Widget-specific state
    │   │   │   └── tree-state.ts
    │   │   └── index.ts
    │   ├── constraint-panel/
    │   │   ├── ui/
    │   │   │   ├── ConstraintPanel.tsx
    │   │   │   └── ConstraintPanel.module.css
    │   │   └── index.ts
    │   ├── preview-panel/
    │   │   ├── ui/
    │   │   │   ├── PreviewPanel.tsx
    │   │   │   ├── JsonPreview.tsx
    │   │   │   └── FshPreview.tsx
    │   │   └── index.ts
    │   ├── diagnostics-panel/
    │   │   ├── ui/
    │   │   │   └── DiagnosticsPanel.tsx
    │   │   └── index.ts
    │   ├── quick-actions-toolbar/
    │   │   ├── ui/
    │   │   │   ├── QuickActionsToolbar.tsx
    │   │   │   └── ElementContextMenu.tsx
    │   │   └── index.ts
    │   ├── resource-explorer/
    │   │   ├── ui/
    │   │   │   └── ResourceExplorer.tsx
    │   │   └── index.ts
    │   └── new-project-wizard/
    │       ├── ui/
    │       │   ├── NewProjectWizard.tsx
    │       │   └── TemplateSelector.tsx
    │       └── index.ts
    │
    ├── features/                    # Layer: Features (user actions)
    │   ├── edit-cardinality/
    │   │   ├── ui/
    │   │   │   ├── CardinalityEditor.tsx
    │   │   │   └── CardinalityEditor.module.css
    │   │   ├── model/
    │   │   │   └── cardinality.ts   # Effector events/effects
    │   │   ├── lib/
    │   │   │   └── validation.ts    # Cardinality validation logic
    │   │   └── index.ts
    │   ├── edit-flags/
    │   │   ├── ui/
    │   │   │   └── FlagsEditor.tsx
    │   │   ├── model/
    │   │   │   └── flags.ts
    │   │   └── index.ts
    │   ├── edit-binding/
    │   │   ├── ui/
    │   │   │   ├── BindingEditor.tsx
    │   │   │   └── ValueSetPicker.tsx
    │   │   ├── model/
    │   │   │   └── binding.ts
    │   │   └── index.ts
    │   ├── add-slice/
    │   │   ├── ui/
    │   │   │   ├── SliceWizard.tsx
    │   │   │   ├── DiscriminatorStep.tsx
    │   │   │   └── SliceWizard.module.css
    │   │   ├── model/
    │   │   │   └── slicing.ts
    │   │   └── index.ts
    │   ├── add-extension/
    │   │   ├── ui/
    │   │   │   └── ExtensionPicker.tsx
    │   │   ├── model/
    │   │   │   └── extension.ts
    │   │   └── index.ts
    │   ├── quick-constrain/
    │   │   ├── ui/
    │   │   │   └── QuickConstraintButtons.tsx
    │   │   ├── model/
    │   │   │   └── quick-actions.ts # makeRequired, prohibit, etc.
    │   │   ├── lib/
    │   │   │   └── shortcuts.ts     # Keyboard shortcut definitions
    │   │   └── index.ts
    │   ├── import-resource/
    │   │   ├── ui/
    │   │   │   └── ImportDialog.tsx
    │   │   ├── model/
    │   │   │   └── import.ts
    │   │   └── index.ts
    │   ├── export-resource/
    │   │   ├── ui/
    │   │   │   └── ExportDialog.tsx
    │   │   ├── model/
    │   │   │   └── export.ts
    │   │   └── index.ts
    │   ├── undo-redo/
    │   │   ├── model/
    │   │   │   └── history.ts
    │   │   └── index.ts
    │   └── manage-favorites/
    │       ├── ui/
    │       │   └── FavoritesPanel.tsx
    │       ├── model/
    │       │   └── favorites.ts
    │       └── index.ts
    │
    ├── entities/                    # Layer: Business entities
    │   ├── project/
    │   │   ├── ui/
    │   │   │   ├── ProjectCard.tsx
    │   │   │   └── ProjectBadge.tsx
    │   │   ├── model/
    │   │   │   ├── project.ts       # $project, $resources stores
    │   │   │   ├── events.ts
    │   │   │   └── effects.ts
    │   │   ├── api/
    │   │   │   └── projectApi.ts
    │   │   └── index.ts
    │   ├── profile/
    │   │   ├── ui/
    │   │   │   ├── ProfileCard.tsx
    │   │   │   └── ProfileBadge.tsx
    │   │   ├── model/
    │   │   │   ├── profile.ts       # $currentProfile store
    │   │   │   ├── events.ts
    │   │   │   └── effects.ts
    │   │   ├── api/
    │   │   │   └── profileApi.ts
    │   │   └── index.ts
    │   ├── element/
    │   │   ├── ui/
    │   │   │   ├── ElementRow.tsx
    │   │   │   ├── ElementBadges.tsx
    │   │   │   └── ElementPath.tsx
    │   │   ├── model/
    │   │   │   └── element.ts       # $selectedElement, element helpers
    │   │   ├── lib/
    │   │   │   ├── element-utils.ts
    │   │   │   └── path-utils.ts
    │   │   └── index.ts
    │   ├── package/
    │   │   ├── ui/
    │   │   │   ├── PackageCard.tsx
    │   │   │   └── PackageList.tsx
    │   │   ├── model/
    │   │   │   └── packages.ts
    │   │   ├── api/
    │   │   │   └── packageApi.ts
    │   │   └── index.ts
    │   ├── valueset/
    │   │   ├── ui/
    │   │   │   └── ValueSetCard.tsx
    │   │   ├── model/
    │   │   │   └── valueset.ts
    │   │   └── index.ts
    │   ├── extension/
    │   │   ├── ui/
    │   │   │   └── ExtensionCard.tsx
    │   │   ├── model/
    │   │   │   └── extension.ts
    │   │   └── index.ts
    │   └── diagnostic/
    │       ├── ui/
    │       │   ├── DiagnosticItem.tsx
    │       │   └── DiagnosticIcon.tsx
    │       ├── model/
    │       │   └── diagnostics.ts
    │       └── index.ts
    │
    └── shared/                      # Layer: Shared infrastructure
        ├── api/
        │   ├── client.ts            # Fetch wrapper, error handling
        │   ├── types.ts             # Generated API types
        │   └── index.ts
        ├── ui/                      # Reusable UI components (no business logic)
        │   ├── tree/
        │   │   ├── VirtualizedTree.tsx
        │   │   └── VirtualizedTree.module.css
        │   ├── modal/
        │   │   └── Modal.tsx
        │   ├── badge/
        │   │   └── Badge.tsx
        │   ├── loading/
        │   │   └── LoadingSpinner.tsx
        │   ├── code-editor/
        │   │   └── CodeEditor.tsx   # Monaco-based editor
        │   └── index.ts
        ├── lib/                     # Pure utilities
        │   ├── fhir/
        │   │   ├── cardinality.ts
        │   │   ├── types.ts
        │   │   └── paths.ts
        │   ├── keyboard/
        │   │   └── shortcuts.ts
        │   └── storage/
        │       └── localStorage.ts
        ├── config/
        │   ├── api.ts               # API base URL
        │   └── theme.ts             # Mantine theme config
        └── types/
            ├── fhir.ts              # FHIR type definitions
            ├── profile.ts
            └── api.ts
```

### 13.4 Import Rules (FSD Public API)

Each slice exports only what's needed via `index.ts`:

```typescript
// features/edit-cardinality/index.ts
export { CardinalityEditor } from './ui/CardinalityEditor';
export { cardinalityUpdated, updateCardinalityFx } from './model/cardinality';
// Internal implementation details are NOT exported

// Usage in widgets/constraint-panel/ui/ConstraintPanel.tsx
import { CardinalityEditor } from '@/features/edit-cardinality';
import { FlagsEditor } from '@/features/edit-flags';
import { BindingEditor } from '@/features/edit-binding';
```

### 13.5 Cross-Slice Communication (Effector)

Features communicate via shared entity models:

```typescript
// entities/element/model/element.ts
import { createStore, createEvent, sample } from 'effector';

// Shared events that features can react to
export const elementSelected = createEvent<string>();
export const elementConstraintChanged = createEvent<ConstraintChange>();

export const $selectedElementId = createStore<string | null>(null)
  .on(elementSelected, (_, id) => id);

// features/edit-cardinality/model/cardinality.ts
import { sample } from 'effector';
import { elementConstraintChanged } from '@/entities/element';

export const cardinalityUpdated = createEvent<CardinalityUpdate>();

// When cardinality is updated, notify the element entity
sample({
  clock: cardinalityUpdated,
  fn: (update) => ({ type: 'cardinality', ...update }),
  target: elementConstraintChanged,
});
```

---

## 14. Low-Code UX Design Principles

### 14.1 Core Philosophy: "Show, Don't Tell"

The UI should make FHIR profiling **visual and intuitive**, hiding complexity while preserving power:

| Principle | Implementation |
|-----------|---------------|
| **Visual over textual** | See element tree, not raw JSON |
| **Guided over freeform** | Wizards for complex tasks (slicing, extensions) |
| **Constrained over open** | Only valid options shown (no invalid cardinalities) |
| **Immediate feedback** | See changes reflected instantly in preview |
| **Progressive disclosure** | Simple tasks first, advanced options on demand |
| **Undo-friendly** | Every action reversible, no fear of mistakes |

### 14.2 Low-Code Interaction Patterns

#### Pattern 1: Visual Element Tree

Instead of editing JSON paths, users interact with a **visual tree**:

```
┌─────────────────────────────────────────────────────────────────────┐
│  📦 Patient (US Core Patient)                                       │
├─────────────────────────────────────────────────────────────────────┤
│  ├── 📋 identifier         1..*  MS  ⚠ Has slices                  │
│  │   ├── 🏷️ identifier:SSN   0..1  MS  [system = urn:oid:2.16...]  │
│  │   └── 🏷️ identifier:MRN   1..1  MS  [system = http://hospi...]  │
│  ├── 📋 name               1..*  MS                                 │
│  │   ├── 📄 family         1..1  MS                                 │
│  │   └── 📄 given          1..*  MS                                 │
│  ├── 📋 telecom            0..*                                     │
│  ├── 📋 gender             1..1  MS  → AdministrativeGender         │
│  ├── 📋 birthDate          1..1  MS                                 │
│  └── 📋 address            0..*  MS                                 │
│      └── 🧩 extension:...   0..1     [US Core Race Extension]       │
└─────────────────────────────────────────────────────────────────────┘
         │         │      │
         │         │      └── Visual badges (MS, bindings, slices)
         │         └── Cardinality shown inline
         └── Icons indicate element type (complex, primitive, extension)
```

#### Pattern 2: Click-to-Edit Constraint Panel

Selecting an element opens a **constraint panel** with form controls:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Patient.identifier                                            [X] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Cardinality                                                        │
│  ┌─────────┐    ┌─────────┐                                        │
│  │ Min: 1  │    │ Max: *  │    [Make Required] [Prohibit]          │
│  └─────────┘    └─────────┘                                        │
│                                                                     │
│  Flags                                                              │
│  [✓] Must Support    [ ] Is Modifier    [ ] Is Summary             │
│                                                                     │
│  Type Constraints                                                   │
│  [Identifier]  ← Only valid types shown                            │
│                                                                     │
│  Documentation                                                      │
│  Short: [Patient identifiers including SSN and MRN    ]            │
│  Definition: [A unique identifier for the patient...  ]            │
│                                                                     │
├─────────────────────────────────────────────────────────────────────┤
│  [+ Add Slice]  [+ Add Extension]  [+ Add Constraint]              │
└─────────────────────────────────────────────────────────────────────┘
```

#### Pattern 3: Guided Wizards for Complex Tasks

**Slicing Wizard** (step-by-step):

```
┌─────────────────────────────────────────────────────────────────────┐
│  Add Slice to: Patient.identifier                                   │
├─────────────────────────────────────────────────────────────────────┤
│  Step 2 of 4: Choose Discriminator                                  │
│                                                                     │
│  How should slices be distinguished?                                │
│                                                                     │
│  ○ By fixed value                                                   │
│     Example: identifier.system = "http://example.org/mrn"          │
│                                                                     │
│  ● By pattern                           ← Recommended for Identifier│
│     Example: identifier matching pattern { system: "..." }          │
│                                                                     │
│  ○ By type                                                          │
│     Example: value[x] as Quantity vs value[x] as CodeableConcept   │
│                                                                     │
│  ○ By profile                                                       │
│     Example: observation conforming to VitalSigns profile          │
│                                                                     │
│  ┌───────────────────────────────────────────────────────────────┐ │
│  │ 💡 Tip: For Identifier elements, "pattern" on system is       │ │
│  │    most common. Each slice will match a specific system URI.  │ │
│  └───────────────────────────────────────────────────────────────┘ │
│                                                                     │
│                            [← Back]  [Next: Define Slice →]         │
└─────────────────────────────────────────────────────────────────────┘
```

**Extension Picker** (search + filter):

```
┌─────────────────────────────────────────────────────────────────────┐
│  Add Extension to: Patient                                          │
├─────────────────────────────────────────────────────────────────────┤
│  🔍 [Search extensions...                           ]               │
│                                                                     │
│  Filter by: [All] [US Core] [Core FHIR] [Project]  [Favorites ★]   │
│                                                                     │
│  ┌───────────────────────────────────────────────────────────────┐ │
│  │ ★ US Core Race Extension                                      │ │
│  │   http://hl7.org/fhir/us/core/StructureDefinition/us-core-race│ │
│  │   Context: Patient | RelatedPerson                            │ │
│  │   ✓ Compatible with current element                     [Add] │ │
│  ├───────────────────────────────────────────────────────────────┤ │
│  │ ★ US Core Ethnicity Extension                                 │ │
│  │   http://hl7.org/fhir/us/core/StructureDefinition/us-core-... │ │
│  │   Context: Patient | RelatedPerson                            │ │
│  │   ✓ Compatible with current element                     [Add] │ │
│  ├───────────────────────────────────────────────────────────────┤ │
│  │   Patient Animal Extension                                    │ │
│  │   http://hl7.org/fhir/StructureDefinition/patient-animal      │ │
│  │   Context: Patient                                            │ │
│  │   ⚠ Rarely used in US implementations                   [Add] │ │
│  └───────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
```

#### Pattern 4: Inline Value Set Binding

```
┌─────────────────────────────────────────────────────────────────────┐
│  Terminology Binding: Patient.gender                                │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Binding Strength:  ● Required ○ Extensible ○ Preferred ○ Example  │
│                                                                     │
│  Value Set:                                                         │
│  ┌───────────────────────────────────────────────────────────────┐ │
│  │ 🔍 [administrative-gender                        ]   [Browse] │ │
│  └───────────────────────────────────────────────────────────────┘ │
│                                                                     │
│  Current: AdministrativeGender                                      │
│  http://hl7.org/fhir/ValueSet/administrative-gender                │
│                                                                     │
│  Preview codes:                                                     │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │  male      Male      Administrative gender: Male           │    │
│  │  female    Female    Administrative gender: Female         │    │
│  │  other     Other     Administrative gender: Other          │    │
│  │  unknown   Unknown   Administrative gender: Unknown        │    │
│  └────────────────────────────────────────────────────────────┘    │
│                                                                     │
│  Recent:  [USCoreRace ★] [AdministrativeGender] [ContactPurpose]   │
│                                                                     │
│                                              [Cancel]  [Apply]      │
└─────────────────────────────────────────────────────────────────────┘
```

### 14.3 Immediate Feedback Loop

Every edit triggers immediate visual feedback:

```
User Action                    Immediate Feedback
─────────────────────────────────────────────────────────────────────
Click "Make Required"    →    Tree shows "1..*", element highlighted
                              Preview panel updates JSON
                              Diagnostics clear if was error

Add new slice            →    Slice appears in tree under parent
                              Slice wizard opens for configuration
                              Validation runs, warnings shown if any

Change binding           →    Binding badge updates on element
                              Preview shows updated SD
                              Cross-reference validation runs
```

### 14.4 Error Prevention (Not Just Error Reporting)

| Scenario | Prevention Strategy |
|----------|---------------------|
| Invalid cardinality (e.g., min > max) | Disable invalid options, show only valid choices |
| Incompatible extension context | Filter extensions to show only compatible ones |
| Duplicate slice names | Validate on input, suggest unique names |
| Broken references | Autocomplete from resolved packages, warn on broken |
| Required base violated | Show base constraints, prevent violation |

### 14.5 Contextual Help System

```
┌─────────────────────────────────────────────────────────────────────┐
│ 💡 Slicing: identifier                                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│ Slicing divides a repeating element into named groups.              │
│                                                                     │
│ Common patterns for Identifier:                                     │
│ • SSN slice: system = "urn:oid:2.16.840.1.113883.4.1"              │
│ • MRN slice: system = "http://hospital.example.org/mrn"            │
│                                                                     │
│ ┌─────────────────────────────────────────────────────────────────┐│
│ │ Example FSH:                                                    ││
│ │                                                                 ││
│ │ * identifier ^slicing.discriminator.type = #pattern             ││
│ │ * identifier ^slicing.discriminator.path = "system"             ││
│ │ * identifier contains SSN 0..1 and MRN 1..1                     ││
│ └─────────────────────────────────────────────────────────────────┘│
│                                                                     │
│ [📖 FHIR Spec] [📖 FSH Docs] [Hide]                                 │
└─────────────────────────────────────────────────────────────────────┘
```

### 14.6 Keyboard-First Power Users

While the UI is mouse-friendly, power users can work entirely via keyboard:

| Shortcut | Action |
|----------|--------|
| `↑/↓` | Navigate element tree |
| `Enter` | Open constraint panel for selected |
| `Ctrl+R` | Make selected element required |
| `Ctrl+0` | Prohibit selected element |
| `Ctrl+M` | Toggle Must Support |
| `Ctrl+E` | Add extension to selected |
| `Ctrl+S` | Add slice to selected |
| `Ctrl+B` | Open binding picker |
| `Ctrl+Z` | Undo |
| `Ctrl+Shift+Z` | Redo |
| `Ctrl+P` | Toggle preview panel |
| `/` | Focus search |
| `?` | Show keyboard shortcuts help |

---

## 15. UI Technology Stack (Selected)

| Technology | Purpose |
|------------|---------|
| **React 18** | UI framework |
| **Vite** | Build tool, dev server |
| **Mantine 7** | Component library (modals, buttons, inputs, etc.) |
| **CSS Modules** | Scoped styling, co-located with components |
| **Effector** | State management (reactive, predictable) |
| **React Router 6** | Client-side routing |
| **react-vtree** | Virtualized tree for large element lists |
| **patronum** | Effector utilities (debounce, etc.) |
| **TypeScript** | Type safety |

---

## 16. UI State Model (Effector)

```typescript
// stores/profile/model.ts
import { createStore, createEvent, createEffect, sample, combine } from 'effector';
import { profileApi } from '@/api/profiles';

// Types
interface ProfileState {
  id: string;
  name: string;
  baseUrl: string;
  elementTree: ElementTreeNode | null;
  canUndo: boolean;
  canRedo: boolean;
}

// Events
export const profileCreated = createEvent<CreateProfileRequest>();
export const elementSelected = createEvent<string>();
export const constraintUpdated = createEvent<UpdateConstraintRequest>();
export const undoTriggered = createEvent();
export const redoTriggered = createEvent();

// Effects (async API calls)
export const createProfileFx = createEffect(profileApi.createProfile);
export const fetchProfileFx = createEffect(profileApi.getProfile);
export const updateConstraintFx = createEffect(profileApi.updateConstraint);
export const undoFx = createEffect(profileApi.undo);
export const redoFx = createEffect(profileApi.redo);

// Stores
export const $profile = createStore<ProfileState | null>(null)
  .on(createProfileFx.doneData, (_, profile) => profile)
  .on(fetchProfileFx.doneData, (_, profile) => profile)
  .on(updateConstraintFx.doneData, (state, result) => ({
    ...state!,
    elementTree: result.elementTree,
    canUndo: result.canUndo,
    canRedo: result.canRedo,
  }));

export const $selectedElementId = createStore<string | null>(null)
  .on(elementSelected, (_, id) => id);

export const $selectedElement = combine(
  $profile,
  $selectedElementId,
  (profile, selectedId) => {
    if (!profile?.elementTree || !selectedId) return null;
    return findElement(profile.elementTree, selectedId);
  }
);

// Wire events to effects
sample({
  clock: profileCreated,
  target: createProfileFx,
});

sample({
  clock: constraintUpdated,
  target: updateConstraintFx,
});

sample({
  clock: undoTriggered,
  source: $profile,
  filter: (profile) => profile?.canUndo ?? false,
  fn: (profile) => profile!.id,
  target: undoFx,
});
```

```typescript
// stores/validation/model.ts
import { createStore, createEffect, sample } from 'effector';
import { validationApi } from '@/api/validation';
import { constraintUpdated } from '../profile/model';
import { debounce } from 'patronum';

export const validateFx = createEffect(validationApi.validate);

export const $diagnostics = createStore<Diagnostic[]>([])
  .on(validateFx.doneData, (_, result) => result.diagnostics);

export const $isValidating = validateFx.pending;

// Debounced validation after constraint updates
const debouncedConstraintUpdate = debounce({
  source: constraintUpdated,
  timeout: 300,
});

sample({
  clock: debouncedConstraintUpdate,
  target: validateFx,
});
```

---

## 17. UI Implementation Notes (CSS Modules)

```css
/* components/profile/ElementNode/ElementNode.module.css */
.node {
  display: flex;
  align-items: center;
  padding: var(--mantine-spacing-xs) var(--mantine-spacing-sm);
  cursor: pointer;
  border-radius: var(--mantine-radius-sm);
  transition: background-color 0.1s ease;
}

.node:hover {
  background-color: var(--mantine-color-gray-1);
}

.selected {
  background-color: var(--mantine-color-blue-1);
}

.modified {
  border-left: 3px solid var(--mantine-color-yellow-6);
}

.added {
  border-left: 3px solid var(--mantine-color-green-6);
}

.hasErrors {
  background-color: var(--mantine-color-red-0);
}

.elementName {
  font-family: var(--mantine-font-family-monospace);
  font-size: var(--mantine-font-size-sm);
  flex: 1;
}

.cardinality {
  color: var(--mantine-color-dimmed);
  font-size: var(--mantine-font-size-xs);
  margin-left: var(--mantine-spacing-sm);
}

.types {
  color: var(--mantine-color-blue-7);
  font-size: var(--mantine-font-size-xs);
  margin-left: var(--mantine-spacing-sm);
}

.flags {
  display: flex;
  gap: var(--mantine-spacing-xs);
  margin-left: var(--mantine-spacing-sm);
}
```

---

## 18. Build & Deploy

```bash
# Development: run frontend and backend separately
cd web && npm run dev                    # Vite dev server on :5173
cargo run -p server -- --dev-cors        # API on :3000

# Production: embed UI in Rust binary
cd web && npm run build                  # Creates dist/
cargo build --release -p server          # rust-embed includes dist/
# Result: single binary `target/release/server`

# Or use npm scripts:
npm run dev          # Runs both frontend and backend
npm run build        # Builds both
npm run preview      # Runs production build locally
```

---

## 19. Next Steps

1. **Toolchain alignment (Day 1)**:
   - Align Rust toolchain/MSRV with `maki` (edition 2024) and pin via `rust-toolchain.toml`
   - Confirm `maki-core` path dependency works end-to-end in this workspace

2. **Round-trip spike (Week 1)**:
   - Implement **lossless SD import/export** strategy (preserve unknown JSON fields)
   - Add determinism tests (stable SD JSON output for the same IR)
   - Run against representative profiles (US Core + IPA + mCODE) and record results

3. **First UI vertical slice (Weeks 2–3)**:
   - React scaffold in `web/` + minimal server endpoints (load/import profile → render element tree)
   - Element tree + inspector for cardinality/flags + preview (SD JSON)
   - Undo/redo over explicit operations (no hidden mutations)

4. **Validation parity harness (Week 4)**:
   - Integrate a “publisher parity run” workflow (on-demand and CI-friendly)
   - Start building a parity regression suite (no new false negatives)

---

## Sources

- [FHIR Shorthand Specification](https://build.fhir.org/ig/HL7/fhir-shorthand/)
- [SUSHI GitHub Repository](https://github.com/FHIR/sushi)
- [Firely Forge Product Page](https://fire.ly/products/forge/)
- [Kodjin FHIR Profiler](https://kodjin.com/fhir-profiler/)
- [SMART FRED GitHub](https://github.com/smart-on-fhir/fred)
- [FSH School Documentation](https://fshschool.org/docs/sushi/)
