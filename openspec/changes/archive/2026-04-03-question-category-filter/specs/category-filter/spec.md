# Category Filter Capability

## ADDED Requirements

### Requirement: Category Selection Screen

The system SHALL display a category selection screen between the Menu and Loading states. The screen SHALL show a list of available categories including an "All Categories" option as the default.

#### Scenario: Category Selection Displayed

- **WHEN** the user presses Enter on the Menu screen
- **THEN** the system SHALL display the category selection screen

#### Scenario: All Categories is Default

- **WHEN** the category selection screen is displayed
- **THEN** "All Categories" SHALL be pre-selected as the default option

#### Scenario: User Selects Specific Category

- **WHEN** the user navigates and selects a specific category
- **THEN** the system SHALL use that category when fetching questions

#### Scenario: User Confirms Selection

- **WHEN** the user confirms the category selection
- **THEN** the system SHALL transition to the Loading state and fetch questions

### Requirement: Category Parameter in API Request

The system SHALL pass the selected category ID to the OpenTDB API when fetching questions. When "All Categories" is selected, no category parameter SHALL be included in the API request.

#### Scenario: Fetch with Specific Category

- **WHEN** user selects category ID 17 (Science)
- **THEN** the API request SHALL be `https://opentdb.com/api.php?amount=10&category=17&type=multiple`

#### Scenario: Fetch with All Categories

- **WHEN** user selects "All Categories"
- **THEN** the API request SHALL be `https://opentdb.com/api.php?amount=10&type=multiple` (no category parameter)

### Requirement: Selected Category in Game State

The system SHALL store the selected category in the Game struct to enable display during gameplay.

#### Scenario: Category Stored in Game State

- **WHEN** user confirms category selection
- **THEN** the selected category ID and name SHALL be stored in the Game struct

### Requirement: Category Display During Gameplay

The system SHALL display the selected category name in the question header during gameplay.

#### Scenario: Category Shown in Question Header

- **WHEN** a question is displayed during gameplay
- **THEN** the question header SHALL show the selected category name
