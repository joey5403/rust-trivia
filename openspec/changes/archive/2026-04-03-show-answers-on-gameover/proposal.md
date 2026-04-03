## Why

Currently, when the game ends, users only see their final score without knowing which questions they got right or wrong, or what the correct answers were. This limits the educational value of the trivia game. Users learn better when they can review all questions and understand the correct answers.

## What Changes

- Modify the GameOver screen to display a scrollable list of all questions with:
  - The question text
  - The correct answer
  - An indicator showing if the user answered correctly
- The existing score and performance message remain visible at the top

## Capabilities

### Modified Capabilities

- `game`: The GameOver display behavior changes to show question review instead of just final score

## Impact

- **UI Module** (`src/ui.rs`): Modify `draw_game_over` function to display question list
- **Game Module** (`src/game.rs`): No changes needed - all data already stored in `questions` and `answer_results`
