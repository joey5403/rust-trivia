## 1. Project Setup

- [ ] 1.1 Add `OPENAI_API_KEY`, `OPENAI_MODEL`, and `OPENAI_BASE_URL` environment variable documentation to project README
- [ ] 1.2 Verify `reqwest` is available for HTTP calls to AI API

## 2. Create Locale Module

- [ ] 2.1 Create `src/locale.rs` with `Locale` enum (En, Zh)
- [ ] 2.2 Create `LocaleStrings` struct with all UI text fields
- [ ] 2.3 Implement `LocaleStrings::get(locale: Locale) -> &'static LocaleStrings`
- [ ] 2.4 Add English locale strings for all UI text
- [ ] 2.5 Add Chinese locale strings for all UI text (menu, loading, question, result, game over, footer, performance messages, categories)

## 3. Create Translation Module

- [ ] 3.1 Create `src/translation.rs` with `Translator` struct
- [ ] 3.2 Implement `Translator::new()` to read `OPENAI_API_KEY`, `OPENAI_MODEL`, and `OPENAI_BASE_URL` from env
- [ ] 3.3 Implement `Translator::translate_text(&self, text: &str, to: Locale) -> Result<String>`
- [ ] 3.4 Implement `Translator::translate_question(&self, question: &TriviaQuestion, locale: Locale) -> Result<TriviaQuestion>` for batch translation
- [ ] 3.5 Add proper error handling with `anyhow::Result<String>`
- [ ] 3.6 Add logging for translation failures

## 4. Modify Main Module (CLI Parsing)

- [ ] 4.1 Parse `--lang` and `--language` arguments in `main()`
- [ ] 4.2 Validate locale value, default to English on invalid input
- [ ] 4.3 Print warning message for invalid locale
- [ ] 4.4 Pass `Locale` to `Game::new(locale)`

## 5. Modify Game Module

- [ ] 5.1 Update `Game::new(locale: Locale)` to accept and store locale
- [ ] 5.2 Add `translator: Option<Translator>` field to `Game` struct
- [ ] 5.3 Initialize `Translator` when `locale == Locale::Zh` and `OPENAI_API_KEY` is set
- [ ] 5.4 Log warning if Chinese mode enabled but API key missing
- [ ] 5.5 After fetching questions in `start_game()`, translate questions if locale is Chinese

## 6. Modify UI Module (Locale-Aware Rendering)

- [ ] 6.1 Update all `draw_*` functions to accept `locale: Locale` parameter
- [ ] 6.2 Replace hardcoded text with `LocaleStrings::get(locale).field_name`
- [ ] 6.3 Update `draw_menu` with locale-aware welcome and instructions text
- [ ] 6.4 Update `draw_loading` with locale-aware loading message
- [ ] 6.5 Update `draw_question` with locale-aware question text, category, difficulty
- [ ] 6.6 Update `draw_result` with locale-aware correct/incorrect messages
- [ ] 6.7 Update `draw_game_over` with locale-aware title, score, performance message
- [ ] 6.8 Update `draw_colored_progress` info line format for Chinese ("问题 X/Y | 得分 X/Y")

## 7. Modify Main Game Loop

- [ ] 7.1 Update `run_game` function signature to accept `Locale`
- [ ] 7.2 Pass `locale` to all `ui::draw` calls
- [ ] 7.3 Update `main()` to pass locale from CLI args to `run_game`

## 8. Testing

- [ ] 8.1 Test English mode: `./rust-trivia`
- [ ] 8.2 Test Chinese mode: `./rust-trivia --lang zh` (requires API key for full test)
- [ ] 8.3 Test invalid locale fallback: `./rust-trivia --lang invalid`
- [ ] 8.4 Verify Chinese UI with English questions when API key missing
- [ ] 8.5 Run `cargo fmt` to ensure code formatting
- [ ] 8.6 Run `cargo clippy` to check for warnings
