## Why

The Rust Trivia game currently fetches questions from all categories randomly. Users may want to focus on specific categories (e.g., Science, History, Geography) to test their knowledge in particular areas. Adding category selection enhances the replayability and educational value of the game.

## What Changes

- Add category selection screen before starting a game
- Default to "All Categories" when no selection is made
- Pass selected category ID to OpenTDB API when fetching questions
- Add category field to game state to track selection
- Display selected category in question header during gameplay

## Capabilities

### New Capabilities

- `category-filter`: Allow users to select a question category before starting a game. Supports "All Categories" (default) or a specific category from OpenTDB. Category selection persists during the game session.

## Impact

- **API Module** (`src/api.rs`): Modify `fetch_questions` to accept optional category ID
- **Game Module** (`src/game.rs`): Add `selected_category` field to Game struct, modify `start_game` to pass category to API
- **UI Module** (`src/ui.rs`): Add new `SelectCategory` game state and corresponding UI screen, update question display to show category
