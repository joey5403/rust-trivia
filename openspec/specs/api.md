# API Module Source of Truth

## Overview

OpenTDB API client for fetching trivia questions. Handles HTTP requests, JSON parsing, and provides fallback data structures.

## Module Information

- **File**: `src/api.rs`
- **Lines**: 72
- **Dependencies**: `reqwest`, `serde`, `anyhow`

## Data Structures

### `TriviaResponse`

```rust
#[derive(Debug, Deserialize)]
pub struct TriviaResponse {
    pub response_code: u32,
    pub results: Vec<TriviaQuestion>,
}
```

**Fields**:
- `response_code`: API status code (0 = success)
- `results`: Vector of trivia questions

### `TriviaQuestion`

```rust
#[derive(Debug, Deserialize, Clone)]
pub struct TriviaQuestion {
    pub category: String,
    pub r#type: String,
    pub difficulty: String,
    pub question: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>,
}
```

**Fields**:
- `category`: Question category (e.g., "Science & Nature")
- `r#type`: Question type (always "multiple" for this app)
- `difficulty`: Difficulty level ("easy", "medium", "hard")
- `question`: The question text (may contain HTML entities)
- `correct_answer`: The correct answer (may contain HTML entities)
- `incorrect_answers`: Vector of 3 wrong answers (may contain HTML entities)

## `TriviaApi` Struct

```rust
pub struct TriviaApi {
    client: Client,
}
```

HTTP client wrapper for making API requests.

### Constructor

```rust
impl TriviaApi {
    pub fn new() -> Self
```

Creates a new `TriviaApi` with a fresh `reqwest::Client`.

## Public Methods

### `fetch_questions`

```rust
pub async fn fetch_questions(&self, amount: u32) -> Result<Vec<TriviaQuestion>>
```

**Parameters**:
- `amount`: Number of questions to fetch

**API Endpoint**:
```
https://opentdb.com/api.php?amount={amount}&type=multiple
```

**Response Handling**:
- `response_code == 0`: Success → return `results` vector
- `response_code != 0`: Error → return `anyhow::Error`

**Error Conditions**:
- Network errors (connection refused, DNS failure, etc.)
- API errors (non-zero response code)
- JSON parsing errors

## `TriviaQuestion` Methods

### `get_all_answers`

```rust
pub fn get_all_answers(&self) -> Vec<String>
```

**Behavior**:
1. Clone `incorrect_answers` into mutable vector
2. Append `correct_answer`
3. Sort alphabetically
4. Return combined vector

**Note**: Answers are shuffled (sorted) to randomize position of correct answer.

### `get_correct_index`

```rust
pub fn get_correct_index(&self) -> usize
```

**Behavior**:
1. Call `get_all_answers()` to get sorted answers
2. Find position where answer equals `correct_answer`
3. Return index (0-indexed)

**Returns**: `0` if not found (fallback, should never happen)

## HTML Entity Decoding

Questions and answers from OpenTDB may contain HTML entities:

| Entity | Replacement |
|--------|-------------|
| `&quot;` | `"` |
| `&#039;` | `'` |
| `&amp;` | `&` |
| `&lt;` | `<` |
| `&gt;` | `>` |
| `&apos;` | `'` |

**Note**: HTML decoding is handled in `ui.rs::decode_html()`, NOT in this module. The API module returns raw strings with HTML entities intact.

## Architecture Notes

- Stateless API client (no shared state between requests)
- Uses `reqwest::Client` with default configuration
- No rate limiting or retry logic
- Single API endpoint (no pagination, no category selection)
