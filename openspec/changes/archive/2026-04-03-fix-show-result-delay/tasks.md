## 1. Remove Auto-Wait Delay

- [x] 1.1 Remove `sleep(Duration::from_secs(2)).await` and associated `next_question()` call from `main.rs` lines 95-99
- [x] 1.2 Verify build passes: `cargo build`

## 2. Ignore Input During ShowResult

- [x] 2.1 Add no-op handling for 1-4 keys when `game.state == GameState::ShowResult` (already works - existing match only handles Question state)
- [x] 2.2 Add Enter key handling to advance from ShowResult to next question
