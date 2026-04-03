# AI Translation Specification

## ADDED Requirements

### Requirement: OpenAI API Key Configuration
The translation service SHALL read the API key from the `OPENAI_API_KEY` environment variable.

#### Scenario: API key provided via environment variable
- **WHEN** `OPENAI_API_KEY` environment variable is set
- **THEN** `Translator` uses this key for API requests

#### Scenario: API key missing
- **WHEN** `OPENAI_API_KEY` environment variable is not set
- **THEN** `Translator` returns an error when translation is attempted

### Requirement: Question Translation
When Chinese locale is active, all trivia questions fetched from OpenTDB SHALL be translated to Chinese via the AI translation service.

#### Scenario: Translate question text
- **WHEN** a question with text "What is 2 + 2?" is fetched and locale is `zh`
- **THEN** the question text becomes "2 + 2 等于多少？"

#### Scenario: Translate answer options
- **WHEN** answer options are ["2", "3", "4", "5"] and locale is `zh`
- **THEN** answer options become ["2", "3", "4", "5"] (numbers remain unchanged)

#### Scenario: Translate category name
- **WHEN** question category is "General Knowledge" and locale is `zh`
- **THEN** category becomes "常识知识"

#### Scenario: Translate difficulty level
- **WHEN** question difficulty is "easy" and locale is `zh`
- **THEN** difficulty becomes "简单"

### Requirement: Translation Failure Handling
If translation fails, the application SHALL log a warning and use the original English text.

#### Scenario: Translation API returns error
- **WHEN** AI API call fails with network error
- **THEN** the system logs "Translation failed: <error>" and uses original English text

#### Scenario: Translation API returns empty response
- **WHEN** AI API call succeeds but returns empty text
- **THEN** the system logs "Translation returned empty" and uses original English text

### Requirement: Translation Service Configuration
The translation service SHALL support configuration via environment variables.

#### Scenario: Custom model selection
- **WHEN** `OPENAI_MODEL` environment variable is set to "gpt-4o"
- **THEN** `Translator` uses "gpt-4o" as the model

#### Scenario: Default model when not specified
- **WHEN** `OPENAI_MODEL` environment variable is not set
- **THEN** `Translator` uses "gpt-4o-mini" as the default model

### Requirement: Custom API Base URL Support
The translation service SHALL support custom OpenAI-compatible API endpoints via the `OPENAI_BASE_URL` environment variable.

#### Scenario: Custom base URL via environment variable
- **WHEN** `OPENAI_BASE_URL` environment variable is set (e.g., "https://api.deepseek.com/v1")
- **THEN** `Translator` uses this base URL instead of OpenAI's default "https://api.openai.com/v1"

#### Scenario: Default base URL when not specified
- **WHEN** `OPENAI_BASE_URL` environment variable is not set
- **THEN** `Translator` uses OpenAI's default base URL "https://api.openai.com/v1"

### Requirement: Graceful Degradation Without API Key
When Chinese locale is active but no API key is provided, the application SHALL continue with English questions and display a warning.

#### Scenario: Chinese mode without API key
- **WHEN** user runs `./rust-trivia --lang zh` but `OPENAI_API_KEY` is not set
- **THEN** the application displays "Warning: OPENAI_API_KEY not set, questions will remain in English"
- **AND** the application continues with English questions but Chinese UI

### Requirement: Translation Prompt Format
The translation service SHALL use a structured prompt that instructs the LLM to provide concise, direct translations without explanations.

#### Scenario: Prompt structure for translation
- **WHEN** translating "What is the capital of France?"
- **THEN** the prompt instructs: "Translate to Chinese: What is the capital of France?"
- **AND** the expected response is a single line with the Chinese translation
