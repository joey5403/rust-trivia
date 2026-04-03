# Rust Trivia Game

A terminal-based trivia game built with Rust, featuring a beautiful TUI powered by ratatui and real questions from the OpenTDB API.

## Features

- **Interactive Terminal UI**: Built with ratatui for a polished terminal experience
- **24 Category Selection**: Choose from 24 trivia categories or play all categories
- **Live Trivia Questions**: Fetches real questions from the OpenTDB API
- **Chinese Translation**: Questions can be translated to Chinese via OpenAI API
- **Concurrent Translation**: Fast parallel translation of all questions
- **Score Tracking**: Real-time score display with colorful progress indicators
- **Keyboard Navigation**: Intuitive controls for playing
- **Error Handling**: Graceful fallback with built-in questions when API is unavailable

## Available Categories

- All Categories
- General Knowledge
- Entertainment: Books, Film, Music, Musicals & Theatres, Television, Video Games, Board Games, Comics, Japanese Anime & Manga, Cartoon & Animations
- Science & Nature, Science: Computers, Science: Mathematics, Science: Gadgets
- Mythology, Sports, Geography, History, Politics, Art, Celebrities, Animals, Vehicles

## Technologies Used

- **Rust** - Systems programming language
- **ratatui** - Terminal user interface library
- **crossterm** - Cross-platform terminal manipulation
- **tokio** - Async runtime for Rust
- **reqwest** - HTTP client for API requests
- **serde** - Serialization/deserialization framework
- **anyhow** - Flexible error handling
- **OpenTDB API** - Free trivia questions database
- **OpenAI API** - Optional Chinese translation (requires `OPENAI_API_KEY`)

## Getting Started

### Prerequisites

- Rust (1.70 or later)
- Internet connection (for fetching trivia questions)
- `OPENAI_API_KEY` (optional, for Chinese translation)

### Installation & Running

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rust-trivia
   ```

2. Build and run:
   ```bash
   cargo run --release
   ```

### Command Line Flags

| Flag | Description |
|------|-------------|
| `--lang=zh` | Enable Chinese UI and translation |
| `--debug` | Enable debug logging to `rust-trivia.log` |

Example:
```bash
cargo run --release -- --lang=zh --debug
```

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `OPENAI_API_KEY` | API key for translation | Only for Chinese mode |
| `OPENAI_MODEL` | Model to use (default: `gpt-4o-mini`) | No |
| `OPENAI_BASE_URL` | API base URL | No |
| `LOG_FILE` | Debug log file path (default: `rust-trivia.log`) | No |

## How to Play

1. **Main Menu**: Press `Enter` to start
2. **Select Category**: Use `↑`/`↓` to navigate, `Enter` to confirm (first option = all categories)
3. **Answer Questions**: Press `1-4` to select your answer
4. **View Results**: Final results shown at game over with all questions and correct answers

## Game Flow

1. **Menu Screen**: Welcome screen with instructions
2. **Category Selection**: Browse and select a trivia category
3. **Loading**: Fetches questions from OpenTDB API, translates if Chinese mode enabled
4. **Questions**: Multiple choice questions with 4 shuffled options
5. **Game Over**: Final score, percentage, and detailed question review

## Project Structure

```
src/
├── main.rs      # Application entry point and game loop
├── lib.rs      # Library exports for testing
├── api.rs      # OpenTDB API client and data structures
├── game.rs     # Game state management and logic
├── ui.rs       # Terminal UI rendering with ratatui
├── locale.rs   # Localization strings (English/Chinese)
├── translation.rs  # OpenAI translation client

benches/
└── translation_bench.rs  # Translation speed benchmark
```

## Controls

| Key | Action |
|-----|--------|
| `Enter` | Start game / Confirm category / Play again |
| `↑` / `↓` | Navigate category list |
| `1-4` | Select answer |
| `q` | Quit game |

## Progress Tracking

During the game, the progress bar shows:
- **✓ (green)** - Correct answer
- **✗ (red)** - Incorrect answer  
- **● (yellow)** - Current question
- **○ (gray)** - Upcoming questions

## Performance Messages

Based on your final score percentage:

| Score | Message |
|-------|---------|
| 90%+ | 🏆 Excellent! You're a trivia master! |
| 80-89% | 🌟 Great job! Very impressive! |
| 70-79% | 👍 Good work! Keep it up! |
| 60-69% | 😊 Not bad! Room for improvement! |
| <60% | 😅 Better luck next time! |

## API Integration

The game fetches trivia questions from the [Open Trivia Database](https://opentdb.com/) API:
- Endpoint: `https://opentdb.com/api.php`
- Question Type: Multiple choice only
- Default: 10 questions per game
- Fallback: Built-in questions if API is unavailable

### Translation

When `--lang=zh` is enabled and `OPENAI_API_KEY` is set:
- Questions are translated to Chinese via OpenAI API
- Category names are pre-translated via dictionary lookup
- Difficulty levels are pre-translated (easy→简单, medium→中等, hard→困难)
- All 10 questions are translated concurrently for speed

## Development

### Build
```bash
cargo build              # Debug build
cargo build --release    # Release build
```

### Run
```bash
cargo run                # Run debug build
cargo run --release     # Run release build
```

### Test
```bash
cargo test
```

### Benchmark
```bash
OPENAI_API_KEY=your_key cargo bench
```

### Format
```bash
cargo fmt
```

### Lint
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

## Acknowledgments

- [OpenTDB](https://opentdb.com/) for providing free trivia questions
- [ratatui](https://github.com/ratatui-org/ratatui) for the excellent TUI framework
- The Rust community for amazing libraries and tools
