use crate::game::{get_categories, Game, GameState};
use crate::locale::{Locale, LocaleStrings};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, game: &Game, locale: Locale) {
    let strings = LocaleStrings::get(locale);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let header = Paragraph::new(strings.header_title)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    match game.state {
        GameState::Menu => draw_menu(f, chunks[1], strings),
        GameState::SelectCategory => draw_select_category(f, chunks[1], game, strings),
        GameState::Loading => draw_loading(f, chunks[1], strings),
        GameState::Question => draw_question(f, chunks[1], game, strings),
        GameState::GameOver => draw_game_over(f, chunks[1], game, strings),
    }

    let footer_text = match game.state {
        GameState::Menu => strings.footer_menu,
        GameState::SelectCategory => strings.footer_select_category,
        GameState::Question => strings.footer_question,
        GameState::GameOver => strings.footer_game_over,
        _ => strings.footer_quit,
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn draw_menu(f: &mut Frame, area: ratatui::layout::Rect, strings: &LocaleStrings) {
    let text = vec![
        Line::from(""),
        Line::from(strings.welcome_title),
        Line::from(""),
        Line::from(strings.welcome_subtitle),
        Line::from(""),
        Line::from(strings.menu_instruction),
    ];

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Menu"));
    f.render_widget(paragraph, area);
}

fn draw_select_category(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    game: &Game,
    strings: &LocaleStrings,
) {
    let categories = get_categories();
    let mut items = vec![ListItem::new(format!(
        "{} {}",
        if game.category_index == 0 {
            "● "
        } else {
            "  "
        },
        strings.all_categories
    ))];

    for (i, cat) in categories.iter().enumerate() {
        let marker = if game.category_index == i + 1 {
            "● "
        } else {
            "  "
        };
        items.push(ListItem::new(format!("{}{}", marker, cat.name)));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(strings.select_category_title),
        )
        .style(Style::default());

    f.render_widget(list, area);
}

fn draw_loading(f: &mut Frame, area: ratatui::layout::Rect, strings: &LocaleStrings) {
    let text = vec![
        Line::from(""),
        Line::from(strings.loading_title),
        Line::from(""),
        Line::from(strings.loading_message),
    ];

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Loading"));
    f.render_widget(paragraph, area);
}

fn draw_question(f: &mut Frame, area: ratatui::layout::Rect, game: &Game, strings: &LocaleStrings) {
    if let Some(question) = game.current_question() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Min(8),
            ])
            .split(area);

        draw_colored_progress(f, chunks[0], game, strings);

        let question_text = decode_html(&question.question);
        let question_widget = Paragraph::new(question_text)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{} | {}", question.category, question.difficulty)),
            )
            .alignment(Alignment::Left);
        f.render_widget(question_widget, chunks[1]);

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
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(strings.answers_title),
            )
            .style(Style::default().fg(Color::White));
        f.render_widget(answers_widget, chunks[2]);
    }
}

fn draw_game_over(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    game: &Game,
    strings: &LocaleStrings,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(0)])
        .split(area);

    let score_percent = (game.score as f64 / game.questions.len() as f64) * 100.0;

    let header_text = vec![
        Line::from(strings.game_over_title),
        Line::from(""),
        Line::from(format!(
            "{} {}/{}",
            strings.final_score,
            game.score,
            game.questions.len()
        )),
        Line::from(format!("{} {:.1}%", strings.percentage, score_percent)),
        Line::from(""),
        Line::from(get_performance_message(score_percent, strings)),
    ];

    let header_paragraph = Paragraph::new(header_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Game Over"));
    f.render_widget(header_paragraph, chunks[0]);

    let question_items: Vec<ListItem> = game
        .questions
        .iter()
        .enumerate()
        .map(|(i, q)| {
            let is_correct = game.answer_results.get(i).copied().unwrap_or(false);
            let symbol = if is_correct { "✓" } else { "✗" };
            let color = if is_correct { Color::Green } else { Color::Red };
            let question_text = decode_html(&q.question);
            let correct_answer = decode_html(&q.correct_answer);
            let lines = vec![
                Line::from(vec![
                    Span::styled(format!("[{}] ", symbol), Style::default().fg(color)),
                    Span::raw(format!("Q{}: {}", i + 1, question_text)),
                ]),
                Line::from(vec![
                    Span::raw("      → "),
                    Span::styled(correct_answer, Style::default().fg(Color::Blue)),
                ]),
            ];
            ListItem::new(lines)
        })
        .collect();

    let questions_list = List::new(question_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Questions & Answers"),
        )
        .style(Style::default());

    f.render_widget(questions_list, chunks[1]);
}

fn get_performance_message(percentage: f64, strings: &LocaleStrings) -> &'static str {
    match percentage {
        p if p >= 90.0 => strings.perf_excellent,
        p if p >= 80.0 => strings.perf_great,
        p if p >= 70.0 => strings.perf_good,
        p if p >= 60.0 => strings.perf_not_bad,
        _ => strings.perf_better_luck,
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

fn draw_colored_progress(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    game: &Game,
    strings: &LocaleStrings,
) {
    let (current, total) = game.progress();

    let mut spans = Vec::new();

    for (i, &is_correct) in game.answer_results.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw(" "));
        }

        let color = if is_correct { Color::Green } else { Color::Red };
        let symbol = if is_correct { "✓" } else { "✗" };
        spans.push(Span::styled(symbol, Style::default().fg(color)));
    }

    if game.current_question_index < game.questions.len() {
        if !game.answer_results.is_empty() {
            spans.push(Span::raw(" "));
        }
        spans.push(Span::styled("●", Style::default().fg(Color::Yellow)));

        for _ in (game.current_question_index + 1)..game.questions.len() {
            spans.push(Span::raw(" "));
            spans.push(Span::styled("○", Style::default().fg(Color::Gray)));
        }
    } else {
        for i in game.answer_results.len()..game.questions.len() {
            if i > 0 || !game.answer_results.is_empty() {
                spans.push(Span::raw(" "));
            }
            spans.push(Span::styled("○", Style::default().fg(Color::Gray)));
        }
    }

    let progress_line = Line::from(spans);
    let score_info = format!(
        "Question {}/{} | Score: {}/{}",
        current,
        total,
        game.score,
        game.answer_results.len().max(1)
    );

    let progress_paragraph = Paragraph::new(vec![progress_line, Line::from(score_info)])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(strings.progress_title),
        );

    f.render_widget(progress_paragraph, area);
}
