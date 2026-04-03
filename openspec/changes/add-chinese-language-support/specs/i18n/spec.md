# i18n Specification

## ADDED Requirements

### Requirement: Locale Configuration via CLI
The application SHALL accept a `--lang` or `--language` command-line argument to set the display locale. The default locale SHALL be English (`en`).

#### Scenario: Default English when no flag provided
- **WHEN** user runs `./rust-trivia` without any language flag
- **THEN** the application uses English locale

#### Scenario: Chinese locale with flag
- **WHEN** user runs `./rust-trivia --lang zh`
- **THEN** the application uses Chinese locale

#### Scenario: Invalid locale defaults to English
- **WHEN** user runs `./rust-trivia --lang invalid`
- **THEN** the application logs a warning and uses English locale

### Requirement: Locale-Aware UI Text
All user-facing text strings SHALL be provided by a locale-aware string provider based on the active locale.

#### Scenario: English menu text
- **WHEN** locale is `en` and game state is `Menu`
- **THEN** display "Welcome to Rust Trivia!" and "Press ENTER to start playing"

#### Scenario: Chinese menu text
- **WHEN** locale is `zh` and game state is `Menu`
- **THEN** display "欢迎来到 Rust 答题游戏！" and "按回车键开始游戏"

### Requirement: Supported Locales
The application SHALL support exactly two locales: English (`en`) and Chinese (`zh`).

#### Scenario: English locale constant
- **WHEN** locale is `en`
- **THEN** `Locale::En` enum variant is active

#### Scenario: Chinese locale constant
- **WHEN** locale is `zh`
- **THEN** `Locale::Zh` enum variant is active

### Requirement: Locale Strings Coverage
The locale system SHALL provide Chinese translations for all of the following UI elements:

- Menu welcome text
- Menu instructions
- Loading message
- Question header format (category | difficulty)
- Footer help text for each game state
- Result correct/incorrect messages
- Game over title
- Performance messages (excellent, great, good, not bad, better luck)
- Category names
- Quit confirmation

#### Scenario: All UI text has Chinese translation
- **WHEN** locale is `zh`
- **THEN** every UI string has a corresponding Chinese translation
