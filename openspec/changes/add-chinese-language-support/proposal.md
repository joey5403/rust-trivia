## Why

The Rust Trivia Game currently only supports English, limiting its accessibility to Chinese-speaking users. Adding Chinese language support with AI-powered question translation allows the game to reach a broader global audience while maintaining the default English experience for existing users.

## What Changes

- Add `--lang` / `--language` CLI argument to switch UI language (default: `en`, supported: `zh`)
- Implement i18n (internationalization) infrastructure with locale detection
- Add AI translation service integration for translating trivia questions from OpenTDB API
- Translate all UI text strings to Chinese
- Default to English; Chinese enabled only via explicit CLI flag
- Questions translated on-the-fly via AI API when Chinese mode is active

## Capabilities

### New Capabilities

- `i18n`: Internationalization system providing locale-aware text rendering
  - Locale configuration via CLI argument
  - Text string storage in locale files (English/Chinese)
  - Locale-aware message formatting
- `ai-translation`: AI-powered translation for trivia questions
  - Integration with LLM API (OpenAI-compatible) for translating question text
  - Translation of question, answers, category, and difficulty
  - Graceful fallback to original English text if translation fails

### Modified Capabilities

- `main`: CLI argument parsing to accept `--lang` flag
  - New argument: `--lang <locale>` (default: `en`)
  - Locale passed to Game initialization
- `ui`: All displayed text strings to be locale-aware
  - Menu, loading, question, result, and game over screens
  - Footer help text
  - Performance messages

## Impact

- **Code**: `src/main.rs` (CLI parsing), `src/ui.rs` (i18n text), `src/game.rs` (locale handling), new translation module
- **Dependencies**: New dependency for AI translation (e.g., `reqwest` already exists for API calls)
- **API**: No public API changes; internal translation service integration
- **User-Facing**: Users can now play in Chinese by running `./rust-trivia --lang zh`
