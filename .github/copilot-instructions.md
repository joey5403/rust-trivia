<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

This is a Rust trivia game project using the following technologies:

## Key Libraries & Technologies
- **ratatui**: Terminal User Interface library for creating beautiful TUIs
- **crossterm**: Cross-platform terminal manipulation library
- **tokio**: Async runtime for Rust
- **reqwest**: HTTP client for making API requests
- **serde**: Serialization/deserialization framework
- **OpenTDB API**: Free trivia questions database (https://opentdb.com/)

## Project Structure
- `src/main.rs`: Main application entry point and game loop
- `src/api.rs`: OpenTDB API client and data structures
- `src/game.rs`: Game state management and logic
- `src/ui.rs`: Terminal UI rendering with ratatui

## Code Style Guidelines
- Use async/await for API calls and I/O operations
- Follow Rust naming conventions (snake_case for variables/functions, PascalCase for types)
- Handle errors gracefully with the `anyhow` crate
- Use structured logging where appropriate
- Keep UI components modular and reusable

## API Integration
- Fetch trivia questions from OpenTDB API
- Handle API errors gracefully with fallback questions
- Parse JSON responses into strongly-typed Rust structs
- Support multiple choice questions only

## UI/UX Requirements
- Clean, intuitive terminal interface
- Real-time score tracking
- Progress indicators
- Keyboard navigation (1-4 for answers, Enter for actions, q to quit)
- Color coding for correct/incorrect answers
- Responsive layout that works in different terminal sizes
