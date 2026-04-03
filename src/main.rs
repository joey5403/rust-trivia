mod api;
mod game;
mod locale;
mod translation;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tokio::time::Duration;

use crate::game::{init_log_file, set_debug, Game, GameState};
use crate::locale::Locale;
use std::path::PathBuf;

fn parse_args() -> (Locale, bool) {
    let args: Vec<String> = std::env::args().collect();
    let mut locale = Locale::En;
    let mut debug = false;
    
    for arg in &args {
        if arg == "--lang=zh" || arg == "--language=chinese" {
            locale = Locale::Zh;
        }
        if arg == "--debug" {
            debug = true;
        }
    }
    
    (locale, debug)
}

#[tokio::main]
async fn main() -> Result<()> {
    let (locale, debug) = parse_args();
    
    let log_path = std::env::var("LOG_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("rust-trivia.log"));
    init_log_file(log_path);
    set_debug(debug);
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut game = Game::new(locale).await?;
    let result = run_game(&mut terminal, &mut game, locale).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

async fn run_game<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    game: &mut Game,
    locale: Locale,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, game, locale))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('1'..='4') => {
                        if matches!(game.state, GameState::Question) {
                            if let KeyCode::Char(c) = key.code {
                                let answer = (c as u8 - b'1') as usize;
                                game.answer_question(answer).await?;
                            }
                        }
                    }
                    KeyCode::Enter => match game.state {
                        GameState::Menu => {
                            game.state = GameState::SelectCategory;
                        }
                        GameState::SelectCategory => {
                            game.confirm_category();
                            game.start_game().await?;
                        }
                        GameState::GameOver => {
                            game.reset_game().await?;
                        }
                        _ => {}
                    },
                    KeyCode::Down => {
                        if matches!(game.state, GameState::SelectCategory) {
                            game.navigate_category(true);
                        }
                    }
                    KeyCode::Up => {
                        if matches!(game.state, GameState::SelectCategory) {
                            game.navigate_category(false);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
