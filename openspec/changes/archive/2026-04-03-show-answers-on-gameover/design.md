## Context

The Rust Trivia game has a GameOver screen that shows final score and performance message. The `questions` vector and `answer_results` vector already store all question data and user answers. Currently this data is not displayed on GameOver.

## Goals / Non-Goals

**Goals:**
- Display all questions with correct answers on GameOver screen
- Show user's answer and whether it was correct
- Keep score and performance message visible

**Non-Goals:**
- Scrollable list if questions exceed terminal height (accept clipping)
- Modify game logic or state machine

## Decisions

### 1. GameOver Screen Layout

**Decision**: Replace the centered text layout with a scrollable list layout.

**Rationale**: The existing `draw_game_over` uses `Paragraph` with centered text. To show multiple questions, we need a list widget. Use ratatui's `List` widget to display each question with its correct answer and user result.

### 2. Question Display Format

**Decision**: For each question show: question number, question text (truncated), correct answer, and ✓/✗ indicator.

**Format**: `[✓/✗] Q{n}: {question_text} → {correct_answer}`

**Rationale**: Compact enough to fit multiple questions, clear enough to understand results.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Long question text may not fit terminal width | Use text truncation with ellipsis |
| Many questions may exceed terminal height | Let ratatui handle scrolling naturally |
