## 1. UI Module Changes

- [x] 1.1 Modify `draw_game_over` to use List widget instead of Paragraph
- [x] 1.2 Add helper to format each question result (question, correct answer, user result)
- [x] 1.3 Update footer to indicate scrollable content or "Press ENTER to play again"

## 2. Testing

- [x] 2.1 Verify `cargo build --release` compiles without errors
- [ ] 2.2 Test GameOver shows all questions with correct answers
- [ ] 2.3 Test correct/incorrect indicators are accurate
- [ ] 2.4 Verify score and performance message still visible
- [x] 2.5 Test `cargo clippy` passes with no warnings
