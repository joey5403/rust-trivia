# Game Module

## MODIFIED Requirements

### Requirement: GameOver Display

The system SHALL display the GameOver screen when all questions have been answered. The screen SHALL show final score, performance message, and a list of all questions with correct answers.

#### Scenario: GameOver Shows Results

- **WHEN** all questions have been answered
- **THEN** the system SHALL display the final score, performance message, and a list of all questions with correct answers marked

#### Scenario: Question List Shows Each Question

- **WHEN** the GameOver screen is displayed
- **THEN** for each question the system SHALL show: the question text, the correct answer, and whether the user answered correctly
