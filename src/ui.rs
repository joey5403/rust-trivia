use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::game::{Game, GameState};

pub fn draw(f: &mut Frame, game: &Game) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Header
    let header = Paragraph::new("🧠 Rust Trivia Game 🧠")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Main content
    match game.state {
        GameState::Menu => draw_menu(f, chunks[1]),
        GameState::Loading => draw_loading(f, chunks[1]),
        GameState::Question => draw_question(f, chunks[1], game),
        GameState::ShowResult => draw_result(f, chunks[1], game),
        GameState::GameOver => draw_game_over(f, chunks[1], game),
    }

    // Footer
    let footer_text = match game.state {
        GameState::Menu => "Press ENTER to start • Press 'q' to quit",
        GameState::Question => "Press 1-4 to select answer • Press 'q' to quit",
        GameState::GameOver => "Press ENTER to play again • Press 'q' to quit",
        _ => "Press 'q' to quit",
    };
    
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn draw_menu(f: &mut Frame, area: ratatui::layout::Rect) {
    let text = vec![
        Line::from(""),
        Line::from("Welcome to Rust Trivia!"),
        Line::from(""),
        Line::from("Test your knowledge with questions from OpenTDB"),
        Line::from(""),
        Line::from("Press ENTER to start playing"),
    ];

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Menu"));
    f.render_widget(paragraph, area);
}

fn draw_loading(f: &mut Frame, area: ratatui::layout::Rect) {
    let text = vec![
        Line::from(""),
        Line::from("Loading questions..."),
        Line::from(""),
        Line::from("Please wait while we fetch trivia questions"),
    ];

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Loading"));
    f.render_widget(paragraph, area);
}

fn draw_question(f: &mut Frame, area: ratatui::layout::Rect, game: &Game) {
    if let Some(question) = game.current_question() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Min(8),
            ])
            .split(area);

        // Colorful progress indicator showing individual results
        draw_colored_progress(f, chunks[0], game);

        // Question
        let question_text = decode_html(&question.question);
        let question_widget = Paragraph::new(question_text)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title(
                format!("{} | {}", question.category, question.difficulty)
            ))
            .alignment(Alignment::Left);
        f.render_widget(question_widget, chunks[1]);

        // Answers
        let answers = question.get_all_answers();
        let answer_items: Vec<ListItem> = answers
            .iter()
            .enumerate()
            .map(|(i, answer)| {
                let answer_text = decode_html(answer);
                ListItem::new(format!("{}. {}", i + 1, answer_text))
            })
            .collect();

        let answers_widget = List::new(answer_items)
            .block(Block::default().borders(Borders::ALL).title("Answers"))
            .style(Style::default().fg(Color::White));
        f.render_widget(answers_widget, chunks[2]);
    }
}

fn draw_result(f: &mut Frame, area: ratatui::layout::Rect, game: &Game) {
    if let Some(question) = game.current_question() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Min(8),
            ])
            .split(area);

        // Colorful progress indicator showing individual results
        draw_colored_progress(f, chunks[0], game);

        // Result
        let result_text = if game.last_answer_correct {
            "✅ Correct!"
        } else {
            "❌ Incorrect!"
        };
        let result_color = if game.last_answer_correct {
            Color::Green
        } else {
            Color::Red
        };

        let result_widget = Paragraph::new(result_text)
            .style(Style::default().fg(result_color).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Result"));
        f.render_widget(result_widget, chunks[1]);

        // Show correct answer
        let correct_answer = decode_html(&question.correct_answer);
        let correct_text = format!("The correct answer was: {}", correct_answer);
        let correct_widget = Paragraph::new(correct_text)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Correct Answer"));
        f.render_widget(correct_widget, chunks[2]);
    }
}

fn draw_game_over(f: &mut Frame, area: ratatui::layout::Rect, game: &Game) {
    let score_percent = (game.score as f64 / game.questions.len() as f64) * 100.0;
    
    let text = vec![
        Line::from(""),
        Line::from("🎉 Game Over! 🎉"),
        Line::from(""),
        Line::from(format!("Final Score: {}/{}", game.score, game.questions.len())),
        Line::from(format!("Percentage: {:.1}%", score_percent)),
        Line::from(""),
        Line::from(get_performance_message(score_percent)),
        Line::from(""),
        Line::from("Press ENTER to play again"),
    ];

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Game Over"));
    f.render_widget(paragraph, area);
}

fn get_performance_message(percentage: f64) -> &'static str {
    match percentage {
        p if p >= 90.0 => "🏆 Excellent! You're a trivia master!",
        p if p >= 80.0 => "🌟 Great job! Very impressive!",
        p if p >= 70.0 => "👍 Good work! Keep it up!",
        p if p >= 60.0 => "😊 Not bad! Room for improvement!",
        _ => "😅 Better luck next time!",
    }
}

fn decode_html(text: &str) -> String {
    text.replace("&quot;", "\"")
        .replace("&#039;", "'")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&apos;", "'")
}

fn draw_colored_progress(f: &mut Frame, area: ratatui::layout::Rect, game: &Game) {
    let (current, total) = game.progress();
    
    // Create spans with appropriate colors
    let mut spans = Vec::new();
    
    for (i, &is_correct) in game.answer_results.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw(" "));
        }
        
        let color = if is_correct { Color::Green } else { Color::Red };
        let symbol = if is_correct { "✓" } else { "✗" };
        spans.push(Span::styled(symbol, Style::default().fg(color)));
    }
    
    // Add current question indicator if we're still in the game
    if game.current_question_index < game.questions.len() {
        if !game.answer_results.is_empty() {
            spans.push(Span::raw(" "));
        }
        spans.push(Span::styled("●", Style::default().fg(Color::Yellow))); // Current question
        
        // Add remaining questions
        for _ in (game.current_question_index + 1)..game.questions.len() {
            spans.push(Span::raw(" "));
            spans.push(Span::styled("○", Style::default().fg(Color::Gray)));
        }
    } else {
        // Game is over, show remaining as gray if any
        for i in game.answer_results.len()..game.questions.len() {
            if i > 0 || !game.answer_results.is_empty() {
                spans.push(Span::raw(" "));
            }
            spans.push(Span::styled("○", Style::default().fg(Color::Gray)));
        }
    }
    
    let progress_line = Line::from(spans);
    let score_info = format!("Question {}/{} | Score: {}/{}", current, total, game.score, game.answer_results.len().max(1));
    
    let progress_paragraph = Paragraph::new(vec![
        progress_line,
        Line::from(score_info),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).title("Progress"));
    
    f.render_widget(progress_paragraph, area);
}
