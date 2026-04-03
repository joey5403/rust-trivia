# Main Module Specification

## MODIFIED Requirements

### Requirement: CLI Argument Parsing

The main entry point SHALL parse command-line arguments to configure the application locale.

#### Scenario: Default English locale
- **WHEN** user runs `./rust-trivia` without `--lang` flag
- **THEN** `Locale::En` is passed to `Game::new(Locale::En)`

#### Scenario: Chinese locale via `--lang zh`
- **WHEN** user runs `./rust-trivia --lang zh`
- **THEN** `Locale::Zh` is passed to `Game::new(Locale::Zh)`

#### Scenario: Chinese locale via `--language zh`
- **WHEN** user runs `./rust-trivia --language zh`
- **THEN** `Locale::Zh` is passed to `Game::new(Locale::Zh)`

#### Scenario: Invalid locale value
- **WHEN** user runs `./rust-trivia --lang xx`
- **THEN** system prints warning "Unknown locale 'xx', defaulting to English"
- **AND** `Locale::En` is used

### Requirement: Locale Propagation

The parsed locale SHALL be passed through to all components that require localization.

#### Scenario: Locale passed to Game
- **WHEN** CLI arguments are parsed
- **THEN** `Locale` is stored in `Game` struct

#### Scenario: Locale passed to UI
- **WHEN** `ui::draw` is called
- **THEN** current `Locale` is passed for text rendering
