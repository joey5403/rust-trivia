## Context

The Rust Trivia Game currently only displays English text. The application consists of:
- `src/main.rs` - Entry point, terminal setup, game loop
- `src/ui.rs` - TUI rendering with ratatui
- `src/game.rs` - Game state and logic
- `src/api.rs` - OpenTDB API client

All user-facing text is hardcoded in `ui.rs` and `game.rs`. This includes menu text, question formatting, result messages, and performance feedback.

**Constraints:**
- Must default to English when no language flag is provided
- Chinese mode must be explicitly enabled via `--lang zh`
- AI translation requires an API key (environment variable)
- Questions come from OpenTDB API (English only)

## Goals / Non-Goals

**Goals:**
- Enable Chinese language UI via `--lang zh` CLI flag
- Translate all UI text strings to Chinese
- Use AI (LLM) to translate trivia questions on-the-fly when Chinese mode is active
- Graceful fallback to English if translation fails or API key is missing

**Non-Goals:**
- Support for languages other than English and Chinese (extensibility is out of scope)
- Persistent locale preference (CLI flag only)
- UI string externalization to locale files (strings stay in code for simplicity)
- Runtime language switching (must restart with new flag)

## Decisions

### Decision 1: CLI Argument Parsing

**Choice:** Use Rust's built-in `std::env::args()` for simplicity

**Rationale:** The only new CLI argument is `--lang <locale>`. Adding `clap` or `structopt` adds a dependency for minimal gain. Using `std::env::args()` keeps the project lean.

**Alternative:** Add `clap` crate
- Pros: Better error handling, auto-help, validation
- Cons: Additional dependency, overkill for single flag

### Decision 2: i18n Architecture

**Choice:** Locale strings stored in a `translations` module with `Locale` enum and string maps

**Structure:**
```rust
pub enum Locale { En, Zh }

pub struct LocaleStrings { /* fields for each UI string */ }

impl LocaleStrings {
    pub fn get(locale: Locale) -> &'static LocaleStrings { ... }
}
```

**Rationale:** Self-contained, no external files to manage, compile-time validated, easy to add new languages by extending the enum and string map.

### Decision 3: AI Translation Integration

**Choice:** OpenAI-compatible API via environment variable `OPENAI_API_KEY`

**API Design:** New `src/translation.rs` module with `Translator` struct:
```rust
pub struct Translator { api_key: String, client: Client }
impl Translator {
    pub async fn translate(&self, text: &str, to: Locale) -> Result<String>
}
```

**Translation Flow:**
1. If locale is `En`, skip translation
2. If locale is `Zh`, call LLM API to translate question, answers, category, difficulty
3. On failure, log warning and return original English text

**Rationale:** Reuses existing `reqwest` dependency, OpenAI-compatible APIs are widely available, environment variable is standard pattern for API keys.

### Decision 4: Fallback Strategy

**Choice:** If `OPENAI_API_KEY` is not set in Chinese mode, display a warning but continue with English questions

**Rationale:** Users can still play with English questions even without the API key. The UI will be in Chinese, but questions remain English. This is acceptable behavior.

### Decision 5: Integration Points

**`main.rs` changes:**
1. Parse `--lang` argument (default: `en`)
2. Pass `Locale` to `Game::new(locale)`
3. Pass `Locale` to UI render functions

**`game.rs` changes:**
1. `Game::new(locale: Locale)` - store locale
2. After fetching questions, if locale is `Zh`, translate all questions via `Translator`

**`ui.rs` changes:**
1. All text strings use `LocaleStrings::get(locale)`
2. `draw_*` functions accept `locale: Locale` parameter

### Decision 6: Custom API Base URL Support

**Choice:** Support custom OpenAI-compatible API endpoints via `OPENAI_BASE_URL` environment variable

**Structure:**
```rust
impl Translator {
    fn get_base_url() -> String {
        std::env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string())
    }
}
```

**Rationale:** Many OpenAI-compatible providers (DeepSeek, Groq, Azure OpenAI, etc.) use `OPENAI_BASE_URL` as the standard environment variable. This allows users to use cheaper or self-hosted translation services.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| AI translation API is slow | Show loading indicator during translation; translate questions in parallel |
| AI translation costs money | Users provide their own API key; no cost to project |
| API key missing in Chinese mode | Warn user, continue with English questions |
| Translation quality varies | Use gpt-4o-mini for cost-efficiency; prompt engineering for better results |
| Long questions may exceed token limits | Truncate or split long text if needed |

## Open Questions

1. **Should we cache translated questions?** (Redis/memory cache for repeated questions)
   - Decision: No cache for v1. Keep it simple.

2. **Should we add a `--translate-only` flag to skip UI translation?**
   - Decision: No. If `--lang zh` is set, everything is in Chinese.

3. **Which LLM model to use?**
   - Decision: `gpt-4o-mini` for cost efficiency. Configurable via `OPENAI_MODEL` env var (default: `gpt-4o-mini`).
