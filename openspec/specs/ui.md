# UI Module Source of Truth

## Overview

Terminal UI rendering using ratatui. Handles all screen drawing and input visualization.

## Module Information

- **File**: `src/ui.rs`
- **Lines**: 255
- **Dependencies**: `ratatui`, `crossterm`, `crate::game`

## Layout Structure

### Main Layout (3 rows)

```
┌─────────────────────────────────────┐
│            HEADER (3)              │  "🧠 Rust Trivia Game 🧠"
├─────────────────────────────────────┤
│                                     │
│          CONTENT (Min 0)           │  State-dependent
│                                     │
├─────────────────────────────────────┤
│            FOOTER (3)               │  Context-sensitive help
└─────────────────────────────────────┘
```

**Constraints**:
- Header: `Constraint::Length(3)`
- Content: `Constraint::Min(0)`
- Footer: `Constraint::Length(3)`

## State-Based Rendering

### `draw` Function

```rust
pub fn draw(f: &mut Frame, game: &Game)
```

Dispatches to appropriate draw function based on `game.state`:

| State | Function |
|-------|----------|
| `Menu` | `draw_menu` |
| `Loading` | `draw_loading` |
| `Question` | `draw_question` |
| `ShowResult` | `draw_result` |
| `GameOver` | `draw_game_over` |

## Screen Designs

### Menu Screen (`draw_menu`)

```
┌─────────────────────────────────────┐
│                                     │
│         Welcome to Rust Trivia!     │
│                                     │
│   Test your knowledge with          │
│   questions from OpenTDB            │
│                                     │
│      Press ENTER to start playing   │
│                                     │
└─────────────────────────────────────┘
```

**Widget**: `Paragraph` with `Alignment::Center`

### Loading Screen (`draw_loading`)

```
┌─────────────────────────────────────┐
│                                     │
│         Loading questions...        │
│                                     │
│  Please wait while we fetch        │
│        trivia questions            │
│                                     │
└─────────────────────────────────────┘
```

**Widget**: `Paragraph` with `Alignment::Center`

### Question Screen (`draw_question`)

**Layout** (3 vertical sections):
```
┌─────────────────────────────────────┐
│           PROGRESS (3)              │  "✓ ✗ ● ○ ○"
│  Question 1/10 | Score: 0/1         │
├─────────────────────────────────────┤
│                                     │
│        QUESTION (Min 5)             │  Category | Difficulty
│                                     │
├─────────────────────────────────────┤
│                                     │
│        ANSWERS (Min 8)              │  "1. Answer A"
│                                     │  "2. Answer B"
│                                     │  "3. Answer C"
│                                     │  "4. Answer D"
└─────────────────────────────────────┘
```

**Components**:
- `draw_colored_progress`: Shows ✓ (green) for correct, ✗ (red) for incorrect, ● (yellow) for current, ○ (gray) for remaining
- `Paragraph` with `Wrap::trim = true` for question text
- `List` widget for answer options
- Question title: `"{category} | {difficulty}"`

### Result Screen (`draw_result`)

**Layout**:
```
┌─────────────────────────────────────┐
│           PROGRESS (3)              │
├─────────────────────────────────────┤
│                                     │
│          RESULT (Min 5)             │  "✅ Correct!" (green)
│                                     │  OR "❌ Incorrect!" (red)
│                                     │
├─────────────────────────────────────┤
│                                     │
│       CORRECT ANSWER (Min 8)        │  "The correct answer was: X"
│                                     │
└─────────────────────────────────────┘
```

**Color Coding**:
- Correct: `Color::Green`
- Incorrect: `Color::Red`

### Game Over Screen (`draw_game_over`)

```
┌─────────────────────────────────────┐
│                                     │
│          🎉 Game Over! 🎉           │
│                                     │
│      Final Score: 8/10              │
│        Percentage: 80.0%            │
│                                     │
│     🌟 Great job! Very impressive!   │
│                                     │
│      Press ENTER to play again      │
│                                     │
└─────────────────────────────────────┘
```

**Performance Messages** (`get_performance_message`):

| Percentage | Message |
|------------|---------|
| ≥90% | "🏆 Excellent! You're a trivia master!" |
| 80-89% | "🌟 Great job! Very impressive!" |
| 70-79% | "👍 Good work! Keep it up!" |
| 60-69% | "😊 Not bad! Room for improvement!" |
| <60% | "😅 Better luck next time!" |

## Progress Indicator (`draw_colored_progress`)

### Visual Legend

| Symbol | Color | Meaning |
|--------|-------|---------|
| `✓` | Green | Correct answer |
| `✗` | Red | Incorrect answer |
| `●` | Yellow | Current question |
| `○` | Gray | Unanswered question |

### Example Display

```
✓ ✗ ● ○ ○ ○ ○ ○ ○ ○ 
Question 5/10 | Score: 3/4
```

**Info Line Format**: `"Question {current}/{total} | Score: {score}/{answered}"`

## HTML Decoding (`decode_html`)

Decodes HTML entities in question/answer text:

| Entity | Replacement |
|--------|-------------|
| `&quot;` | `"` |
| `&#039;` | `'` |
| `&amp;` | `&` |
| `&lt;` | `<` |
| `&gt;` | `>` |
| `&apos;` | `'` |

## Footer Text

| State | Text |
|-------|------|
| `Menu` | "Press ENTER to start • Press 'q' to quit" |
| `Question` | "Press 1-4 to select answer • Press 'q' to quit" |
| `GameOver` | "Press ENTER to play again • Press 'q' to quit" |
| Others | "Press 'q' to quit" |

## Styling Constants

### Colors
- Header: `Color::Cyan`
- Correct: `Color::Green`
- Incorrect: `Color::Red`
- Current indicator: `Color::Yellow`
- Unanswered: `Color::Gray`
- Footer text: `Color::Gray`

### Modifiers
- Header text: `Modifier::BOLD`
- Result text: `Modifier::BOLD`

### Alignment
- All `Paragraph` widgets: `Alignment::Center`
- Question block: `Alignment::Left`

## Architecture Notes

- All rendering is stateless (takes `&Game`, not `&mut Game`)
- Layout calculations done fresh each frame
- Uses ratatui's builder pattern for widgets
- No direct terminal manipulation (delegated to ratatui backend)
