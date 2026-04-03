# Game Module Source of Truth

## Overview

Core game state management and logic. Handles game flow, scoring, and question progression.

## Module Information

- **File**: `src/game.rs`
- **Lines**: 118
- **Dependencies**: `anyhow`, `crate::api`

## State Machine

### `GameState` Enum

```rust
#[derive(Debug, Clone)]
pub enum GameState {
    Menu,       // Initial state, waiting to start
    Loading,    // Fetching questions from API
    Question,   // Displaying current question
    ShowResult, // Showing answer result
    GameOver,   // Game finished, showing final score
}
```

### State Transition Diagram

```
Menu --[start_game]--> Loading --[API success]--> Question
                     |
                     +--[API failure]---------> Question (with fallback)
Question --[answer_question]--> ShowResult --[2s delay + next_question]--> Question
                                                           |
                                                           +--[no more questions]--> GameOver
GameOver --[reset_game]--> Menu
```

## Data Structures

### `Game` Struct

```rust
pub struct Game {
    pub state: GameState,
    pub api: TriviaApi,
    pub questions: Vec<TriviaQuestion>,
    pub current_question_index: usize,
    pub score: u32,
    pub total_questions: u32,
    pub last_answer_correct: bool,
    pub selected_answer: Option<usize>,
    pub answer_results: Vec<bool>,
}
```

**Fields**:
- `state`: Current game state
- `api`: API client instance
- `questions`: Vector of fetched questions
- `current_question_index`: Current question position (0-indexed)
- `score`: Number of correct answers
- `total_questions`: Target number of questions (default: 10)
- `last_answer_correct`: Result of most recent answer
- `selected_answer`: Index of user's selected answer
- `answer_results`: History of correct/incorrect results

## Constructor

### `Game::new`

```rust
pub async fn new() -> Result<Self>
```

**Initial State**:
```rust
Self {
    state: GameState::Menu,
    api: TriviaApi::new(),
    questions: Vec::new(),
    current_question_index: 0,
    score: 0,
    total_questions: 10,
    last_answer_correct: false,
    selected_answer: None,
    answer_results: Vec::new(),
}
```

## Public Methods

### `start_game`

```rust
pub async fn start_game(&mut self) -> Result<()>
```

**Behavior**:
1. Set `state = GameState::Loading`
2. Reset `score = 0`
3. Reset `current_question_index = 0`
4. Clear `answer_results`
5. Fetch questions from API via `api.fetch_questions(total_questions)`
6. On success: store questions, set `state = GameState::Question`
7. On failure: create fallback question, set `state = GameState::Question`

**Fallback Question**:
```rust
TriviaQuestion {
    category: "General Knowledge".to_string(),
    r#type: "multiple".to_string(),
    difficulty: "easy".to_string(),
    question: "What is 2 + 2?".to_string(),
    correct_answer: "4".to_string(),
    incorrect_answers: vec!["2".to_string(), "3".to_string(), "5".to_string()],
}
```

### `answer_question`

```rust
pub async fn answer_question(&mut self) -> Result<()>
```

**Parameters**:
- `answer_index`: User's selected answer (0-3)

**Behavior**:
1. Get current question via `current_question()`
2. Calculate correct index via `get_correct_index()`
3. Set `last_answer_correct = (answer_index == correct_index)`
4. Set `selected_answer = Some(answer_index)`
5. If correct: increment `score += 1`
6. Push result to `answer_results`
7. Set `state = GameState::ShowResult`

### `next_question`

```rust
pub async fn next_question(&mut self) -> Result<()>
```

**Behavior**:
1. Increment `current_question_index += 1`
2. Clear `selected_answer = None`
3. If `current_question_index >= questions.len()`: set `state = GameState::GameOver`
4. Else: set `state = GameState::Question`

### `reset_game`

```rust
pub async fn reset_game(&mut self) -> Result<()>
```

**Behavior**:
1. Set `state = GameState::Menu`
2. Clear `questions`
3. Reset `current_question_index = 0`
4. Reset `score = 0`
5. Clear `selected_answer`
6. Clear `answer_results`

## Query Methods

### `current_question`

```rust
pub fn current_question(&self) -> Option<&TriviaQuestion>
```

Returns `questions.get(current_question_index)`.

### `progress`

```rust
pub fn progress(&self) -> (usize, usize)
```

Returns `(current_question_index + 1, questions.len())`.

## Architecture Notes

- All methods are async to support future async operations
- `Game` owns its state; no interior mutability needed
- Fallback question ensures game is always playable even without network
- `answer_results` tracks full history for progress display
