## 1. API Module Changes

- [x] 1.1 Add `category: Option<u32>` parameter to `fetch_questions` method
- [x] 1.2 Update API URL construction to include `&category={id}` when category is Some
- [x] 1.3 Add Category struct with id and name fields

## 2. Game Module Changes

- [x] 2.1 Add `SelectCategory` variant to `GameState` enum
- [x] 2.2 Add `selected_category: Option<Category>` field to `Game` struct
- [x] 2.3 Add `CATEGORIES` constant with list of common categories
- [x] 2.4 Create `select_category` method to handle category selection
- [x] 2.5 Update `start_game` to pass selected category to API
- [x] 2.6 Update `reset_game` to clear selected category

## 3. UI Module Changes

- [x] 3.1 Add `draw_select_category` function with list of categories
- [x] 3.2 Update `draw` function to handle `SelectCategory` state
- [x] 3.3 Update footer text for `SelectCategory` state
- [x] 3.4 Update `draw_question` to show selected category in header
- [x] 3.5 Update `draw_menu` to show "Select Category" prompt instead of immediate start

## 4. Main Loop Changes

- [x] 4.1 Update event handling to support category selection navigation
- [x] 4.2 Add keyboard controls for category selection (up/down arrows or number keys)
- [x] 4.3 Handle Enter to confirm selection and start loading

## 5. Testing

- [x] 5.1 Verify `cargo build --release` compiles without errors
- [ ] 5.2 Test category selection screen displays all categories
- [ ] 5.3 Test "All Categories" default selection works (questions from multiple categories)
- [ ] 5.4 Test specific category selection (e.g., Science) returns only that category
- [ ] 5.5 Test keyboard navigation in category selection (up/down/Enter)
- [ ] 5.6 Test game flow works correctly after category selection
- [ ] 5.7 Test selected category name appears in question header
- [x] 5.8 Test `cargo clippy` passes with no warnings
