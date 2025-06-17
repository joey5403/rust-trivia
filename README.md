# Rust Trivia Game

A terminal-based trivia game built with Rust, using ratatui for the UI and the OpenTDB API for questions.

## Features

- **Interactive Terminal UI**: Beautiful terminal interface built with ratatui
- **Live Trivia Questions**: Fetches real questions from the OpenTDB API
- **Score Tracking**: Real-time score display and progress tracking
- **Keyboard Navigation**: Simple keyboard controls for playing
- **Error Handling**: Graceful fallback when API is unavailable

## Technologies Used

- **Rust** - Systems programming language
- **ratatui** - Terminal user interface library
- **crossterm** - Cross-platform terminal manipulation
- **tokio** - Async runtime for Rust
- **reqwest** - HTTP client for API requests
- **serde** - Serialization/deserialization framework
- **OpenTDB API** - Free trivia questions database

## Getting Started

### Prerequisites

- Rust (1.70 or later)
- Internet connection (for fetching trivia questions)

### Installation

1. Clone this repository:
   ```bash
   git clone <repository-url>
   cd rust-trivia
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the game:
   ```bash
   cargo run
   ```

## How to Play

1. **Start Game**: Press `Enter` at the main menu to start
2. **Answer Questions**: Press `1-4` to select your answer
3. **View Results**: Results are shown automatically after each question
4. **Navigate**: 
   - `Enter` - Start game/Play again
   - `1-4` - Select answers
   - `q` - Quit at any time

## Game Flow

1. **Menu Screen**: Welcome screen with instructions
2. **Loading**: Fetches questions from OpenTDB API
3. **Questions**: Multiple choice questions with 4 options
4. **Results**: Shows if answer was correct/incorrect
5. **Game Over**: Final score and performance message

## Project Structure

```
src/
├── main.rs     # Main application entry point and game loop
├── api.rs      # OpenTDB API client and data structures
├── game.rs     # Game state management and logic
└── ui.rs       # Terminal UI rendering with ratatui
```

## API Integration

The game fetches trivia questions from the [Open Trivia Database](https://opentdb.com/) API:
- Endpoint: `https://opentdb.com/api.php`
- Question Type: Multiple choice only
- Default: 10 questions per game
- Fallback: Local questions if API is unavailable

## Controls

| Key | Action |
|-----|--------|
| `Enter` | Start game / Play again |
| `1` | Select first answer |
| `2` | Select second answer |
| `3` | Select third answer |
| `4` | Select fourth answer |
| `q` | Quit game |

## Performance Messages

Based on your final score percentage:
- 90%+: "🏆 Excellent! You're a trivia master!"
- 80-89%: "🌟 Great job! Very impressive!"
- 70-79%: "👍 Good work! Keep it up!"
- 60-69%: "😊 Not bad! Room for improvement!"
- <60%: "😅 Better luck next time!"

## Development

### Building for Debug
```bash
cargo build
```

### Building for Release
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Submit a pull request

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

- [OpenTDB](https://opentdb.com/) for providing free trivia questions
- [ratatui](https://github.com/ratatui-org/ratatui) for the excellent TUI framework
- The Rust community for amazing libraries and tools
