# Translation Progress Bar — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add progress bar to AI translation phase — user sees visual feedback (████░░░░░░░░░░░ 2/10) while questions are being translated.

**Architecture:** Two-phase loading screen. Phase 1 (Fetching): static text. Phase 2 (Translating): text counter + visual progress bar. Game holds `LoadingPhase` enum instead of just `GameState::Loading` during translation.

**Tech Stack:** Rust, ratatui (TUI framework)

---

## File Structure

| File | Changes |
|------|---------|
| `src/game.rs` | Add `LoadingPhase` enum, `TranslationProgress` struct, modify `start_game()` |
| `src/locale.rs` | Add `translating_title`, `translating_format` strings for EN and ZH |
| `src/ui.rs` | Modify `draw_loading()` to accept `LoadingPhase`, render progress bar |

---

## Task 1: Add `LoadingPhase` enum and `TranslationProgress` struct to game.rs

**Files:**
- Modify: `src/game.rs:1-13` (add imports + enum), `src/game.rs:64-77` (add fields to Game struct)

- [ ] **Step 1: Add `LoadingPhase` enum after `GameState` enum (around line 13)**

```rust
#[derive(Debug, Clone)]
pub enum LoadingPhase {
    Fetching,
    Translating(Option<TranslationProgress>),
}

#[derive(Debug, Clone)]
pub struct TranslationProgress {
    pub current: u32,  // 1-indexed count
    pub total: u32,
}
```

- [ ] **Step 2: Add `loading_phase` field to `Game` struct (after line 65)**

```rust
pub struct Game {
    // ... existing fields ...
    pub loading_phase: Option<LoadingPhase>,
}
```

- [ ] **Step 3: Initialize `loading_phase` in `Game::new()` (around line 97)**

Add to the `Ok(Self { ... })` block:
```rust
loading_phase: None,
```

- [ ] **Step 4: Commit**

```bash
git add src/game.rs
git commit -m "feat: add LoadingPhase and TranslationProgress types"
```

---

## Task 2: Modify `start_game()` to track and update progress

**Files:**
- Modify: `src/game.rs:101-153`

- [ ] **Step 1: Set `LoadingPhase::Fetching` when entering loading state (line 102)**

After `self.state = GameState::Loading;` add:
```rust
self.loading_phase = Some(LoadingPhase::Fetching);
```

- [ ] **Step 2: Set `LoadingPhase::Translating` before the translation loop (before line 127)**

Before `if self.locale == Locale::Zh && Translator::is_available() {`, add:
```rust
self.loading_phase = Some(LoadingPhase::Translating(None));
let total = questions.len() as u32;
```

- [ ] **Step 3: Update progress after each successful translation (inside the loop, around line 132)**

After `match translator.translate_question(&q, self.locale).await {` inside the `Ok(tq)` arm, add before `translated.push(tq)`:
```rust
self.loading_phase = Some(LoadingPhase::Translating(Some(TranslationProgress {
    current: translated.len() as u32 + 1,
    total,
})));
```

- [ ] **Step 4: Clear `loading_phase` when entering Question state (line 151)**

After `self.state = GameState::Question;` add:
```rust
self.loading_phase = None;
```

Also clear on error paths (lines 144, 148).

- [ ] **Step 5: Commit**

```bash
git add src/game.rs
git commit -m "feat: track translation progress in start_game()"
```

---

## Task 3: Add locale strings for translating phase

**Files:**
- Modify: `src/locale.rs:38-39` (struct fields), `src/locale.rs:90-91` (EN strings), `src/locale.rs:127-128` (ZH strings)

- [ ] **Step 1: Add fields to `LocaleStrings` struct (after line 39)**

```rust
// Loading
pub loading_title: &'static str,
pub loading_message: &'static str,
pub translating_title: &'static str,
pub translating_format: &'static str,
```

- [ ] **Step 2: Add English strings (after line 91 in ENGLISH static)**

```rust
translating_title: "Translating Questions",
translating_format: "Translating Questions ({}/{})",
```

- [ ] **Step 3: Add Chinese strings (after line 128 in CHINESE static)**

```rust
translating_title: "正在翻译题目",
translating_format: "正在翻译题目 ({}/{})",
```

- [ ] **Step 4: Commit**

```bash
git add src/locale.rs
git commit -m "feat: add translating_title and translating_format locale strings"
```

---

## Task 4: Modify `draw_loading()` to render progress bar

**Files:**
- Modify: `src/ui.rs:1` (add import), `src/ui.rs:35` (pass loading_phase), `src/ui.rs:108-120` (rewrite function)

- [ ] **Step 1: Update import in `ui.rs` line 1 to include `LoadingPhase` and `TranslationProgress`**

Change:
```rust
use crate::game::{get_categories, Game, GameState};
```
To:
```rust
use crate::game::{get_categories, Game, GameState, LoadingPhase, TranslationProgress};
```

- [ ] **Step 2: Pass `loading_phase` to `draw_loading()` call (line 35)**

Change:
```rust
GameState::Loading => draw_loading(f, chunks[1], strings),
```
To:
```rust
GameState::Loading => draw_loading(f, chunks[1], strings, &game.loading_phase),
```

- [ ] **Step 3: Rewrite `draw_loading()` function (lines 108-120)**

Replace the function with:

```rust
fn draw_loading(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    strings: &LocaleStrings,
    phase: &Option<LoadingPhase>,
) {
    let text = match phase {
        None | Some(LoadingPhase::Fetching) => vec![
            Line::from(""),
            Line::from(strings.loading_title),
            Line::from(""),
            Line::from(strings.loading_message),
        ],
        Some(LoadingPhase::Translating(progress)) => {
            let (current, total, bar) = match progress {
                Some(p) => {
                    let filled = p.current;
                    let empty = p.total.saturating_sub(p.current);
                    let bar: String = "█".repeat(filled as usize)
                        + &"░".repeat(empty as usize);
                    (p.current, p.total, bar)
                }
                None => (0, 0, String::new()),
            };
            vec![
                Line::from(""),
                Line::from(strings.translating_title),
                Line::from(""),
                Line::from(format!(
                    "{}{} {}/{}",
                    bar,
                    current,
                    total
                )),
            ]
        }
    };

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Loading"));
    f.render_widget(paragraph, area);
}
```

- [ ] **Step 4: Run build to verify**

```bash
cargo build 2>&1
```

Expected: successful build with no errors or warnings

- [ ] **Step 5: Commit**

```bash
git add src/ui.rs
git commit -m "feat: render progress bar during translation phase"
```

---

## Task 5: Verify end-to-end

**Files:**
- Test: Manual verification

- [ ] **Step 1: Run with Chinese locale and API key to see progress bar**

```bash
OPENAI_API_KEY=your_key cargo run -- --lang=zh
```

- [ ] **Step 2: Verify behavior**

- [ ] **Step 6: Commit any final changes**

```bash
git add -A && git commit -m "feat: add translation progress bar"
```
