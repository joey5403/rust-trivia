# Main Module Source of Truth

## Overview

Entry point for the Rust Trivia Game application. Sets up the terminal UI environment and runs the main game loop.

## Module Information

- **File**: `src/main.rs`
- **Lines**: 87
- **Dependencies**: `ratatui`, `crossterm`, `tokio`, `anyhow`

## Terminal Setup

### Initialization Sequence
1. Enable raw mode via `crossterm::terminal::enable_raw_mode()`
2. Get stdout handle and enter alternate screen via `crossterm::execute!()`
3. Enable mouse capture for terminal
4. Create `CrosstermBackend` for ratatui
5. Initialize `Terminal` instance

### Shutdown Sequence
1. Disable raw mode via `crossterm::terminal::disable_raw_mode()`
2. Leave alternate screen
3. Disable mouse capture
4. Show cursor

## Async Runtime

- Uses `#[tokio::main]` async runtime
- Runtime configured with `features = ["full"]`

## Game Loop (`run_game`)

**Signature**:
```rust
async fn run_game<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    game: &mut Game,
) -> Result<()>
```

### Loop Behavior

1. **Draw Phase**: `terminal.draw(|f| ui::draw(f, game))?`
2. **Input Polling**: `event::poll(Duration::from_millis(100))?`
3. **Input Handling**: `event::read()?`
4. **Auto-advance**: After 2 second delay when in `ShowResult` state

### Key Bindings

| Key | Action | Condition |
|-----|--------|-----------|
| `q` | Quit game | Any state |
| `1`-`4` | Select answer | `GameState::Question` |
| `Enter` | Start game | `GameState::Menu` |
| `Enter` | Reset/Play again | `GameState::GameOver` |

### State Transitions Triggered

- `KeyCode::Char('1'..='4')` → calls `game.answer_question(answer).await?`
- `KeyCode::Enter` on `Menu` → calls `game.start_game().await?`
- `KeyCode::Enter` on `GameOver` → calls `game.reset_game().await?`
- `GameState::ShowResult` → after 2s delay, calls `game.next_question().await?`

## Entry Point (`main`)

```rust
#[tokio::main]
async fn main() -> Result<()>
```

### Initialization Steps
1. Create new `Game` instance via `Game::new().await?`
2. Call `run_game(&mut terminal, &mut game).await`
3. Restore terminal state (even on error)
4. Return result

## Error Handling

- Uses `anyhow::Result<()>` for error propagation
- Terminal state is ALWAYS restored, even on error (via explicit restore before returning)

## Architecture Notes

- Backend-agnostic game loop via `ratatui::backend::Backend` trait
- Non-blocking input polling with 100ms timeout
- Automatic state transitions handled in main loop, not in `Game` struct
