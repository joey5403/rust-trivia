# Translation Progress Bar — Design Spec

## Context

- **Problem**: AI translation of OpenTDB questions shows no progress — user sees a static "Translating Questions..." screen with no feedback on how many questions have been translated.
- **Decision Date**: 2026-04-03
- **Status**: Approved

---

## Design

### UX Flow (Two-Phase Loading)

**State 1 — Fetching Questions**
- Text only: `Fetching questions...`
- No progress bar (unknown total)

**State 2 — Translating Questions**
- Text: `Translating Questions (2/10)`
- Visual bar: `████░░░░░░░░░░░`
- Bar format: filled block (█) for completed, empty block (░) for pending

### Data Model

```rust
struct TranslationProgress {
    current: u32,       // 1-indexed count
    total: u32,         // total questions to translate
}

enum LoadingPhase {
    Fetching,
    Translating(Option<TranslationProgress>),  // None = no progress yet
}
```

### UI Changes

**`draw_loading()` signature change:**
```rust
// Before
fn draw_loading(Canvas, x, y, width)

// After  
fn draw_loading(Canvas, x, y, width, phase: LoadingPhase)
```

**Locale strings added:**
- `translating_title` — "Translating Questions"
- `translating_format` — "Translating Questions ({current}/{total})"

### Error Handling

- Skip failed translations silently
- Continue with remaining questions
- No user-facing error messages

---

## Implementation Scope

- Add `LoadingPhase` enum to `game.rs`
- Add `TranslationProgress` struct to `game.rs`  
- Modify `translate_questions()` to track progress and update state
- Add locale strings in `locale.rs`
- Modify `draw_loading()` in `ui.rs` to render progress bar
