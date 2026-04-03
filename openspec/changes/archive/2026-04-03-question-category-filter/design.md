## Context

The Rust Trivia game is a terminal-based TUI application using ratatui and the OpenTDB API. Currently, questions are fetched randomly from all categories. The game has 5 states: Menu → Loading → Question → ShowResult → GameOver.

**Current API endpoint**: `https://opentdb.com/api.php?amount=10&type=multiple`

The OpenTDB API supports a `category` parameter (e.g., `&category=9` for Science). A list endpoint exists at `https://opentdb.com/api_category.php` to fetch available categories, but for simplicity, we will use a hardcoded list of common categories with "All Categories" as default.

## Goals / Non-Goals

**Goals:**
- Allow users to select a question category before starting a game
- Default to "All Categories" when no selection is made
- Display the selected category during gameplay
- Maintain the existing game flow and state machine

**Non-Goals:**
- Fetching category list dynamically from OpenTDB API
- Persisting category preference across sessions
- Adding difficulty filtering (out of scope for this change)

## Decisions

### 1. Category Selection UI: New GameState variant

**Decision**: Add a `SelectCategory` state to `GameState` enum and a corresponding UI function.

**Rationale**: The cleanest approach is to extend the existing state machine. The category selection appears between Menu and Loading states.

**Alternatives considered**:
- Overlay/modal approach: Would complicate the existing draw function with conditional rendering
- Separate function with state flag: Would require tracking both `GameState` and a separate selection flag

### 2. Category Parameter in API

**Decision**: Add `category: Option<u32>` parameter to `fetch_questions`. When `None`, fetch all categories.

**API URL pattern**:
- All: `https://opentdb.com/api.php?amount=10&type=multiple`
- Specific: `https://opentdb.com/api.php?amount=10&category=17&type=multiple`

**Rationale**: OpenTDB uses numeric category IDs. Using `Option<u32>` allows clean representation of "no filter" vs "specific category".

### 3. Hardcoded Category List

**Decision**: Use a static list of common categories in the UI for selection, not fetched from API.

**Rationale**: Fetching categories adds complexity (API call, loading state, error handling). A static list of ~10 popular categories covers 90% of use cases and is simpler to implement.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| OpenTDB category IDs change or differ from hardcoded list | Use numeric IDs that are stable per OpenTDB documentation; add category name to API request for validation |
| User doesn't know which category to select | Default to "All Categories" ensures game always works |
| Category has fewer than 10 questions | OpenTDB returns error code if insufficient questions; show error and retry with fallback |

## Open Questions

1. Should we show the actual category name from the API response in the question header (already done) or only the user-selected category?
2. Should we handle the case where a specific category returns fewer than 10 questions gracefully?
