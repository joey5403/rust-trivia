use crate::api::{Category, TriviaApi, TriviaQuestion};
use crate::locale::Locale;
use crate::translation::{translate_category_name, Translator};
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

static LOG_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);
static DEBUG_ENABLED: Mutex<bool> = Mutex::new(false);

pub fn init_log_file(path: PathBuf) {
    let path_clone = path.clone();
    LOG_PATH.lock().unwrap().replace(path);
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path_clone) {
        let _ = writeln!(f, "=== Game started ===");
    }
}

pub fn set_debug(enabled: bool) {
    *DEBUG_ENABLED.lock().unwrap() = enabled;
}

macro_rules! debug_log {
    ($($arg:tt)*) => {
        if *DEBUG_ENABLED.lock().unwrap() {
            if let Some(path) = LOG_PATH.lock().unwrap().clone() {
                if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path) {
                    let _ = writeln!(f, "[DEBUG] {}", format!($($arg)*));
                }
            }
        }
    };
}

#[derive(Debug, Clone)]
pub enum GameState {
    Menu,
    SelectCategory,
    Loading,
    Question,
    GameOver,
}

#[derive(Debug, Clone)]
pub enum LoadingPhase {
    Fetching,
    Translating(Option<TranslationProgress>),
}

#[derive(Debug, Clone)]
pub struct TranslationProgress {
    pub current: u32, // 1-indexed count
    pub total: u32,
}

pub fn get_categories(locale: Locale) -> Vec<Category> {
    vec![
        Category { id: 9,  name: translate_category_name("General Knowledge", locale) },
        Category { id: 10, name: translate_category_name("Entertainment: Books", locale) },
        Category { id: 11, name: translate_category_name("Entertainment: Film", locale) },
        Category { id: 12, name: translate_category_name("Entertainment: Music", locale) },
        Category { id: 13, name: translate_category_name("Entertainment: Musicals & Theatres", locale) },
        Category { id: 14, name: translate_category_name("Entertainment: Television", locale) },
        Category { id: 15, name: translate_category_name("Entertainment: Video Games", locale) },
        Category { id: 16, name: translate_category_name("Entertainment: Board Games", locale) },
        Category { id: 17, name: translate_category_name("Science & Nature", locale) },
        Category { id: 18, name: translate_category_name("Science: Computers", locale) },
        Category { id: 19, name: translate_category_name("Science: Mathematics", locale) },
        Category { id: 20, name: translate_category_name("Mythology", locale) },
        Category { id: 21, name: translate_category_name("Sports", locale) },
        Category { id: 22, name: translate_category_name("Geography", locale) },
        Category { id: 23, name: translate_category_name("History", locale) },
        Category { id: 24, name: translate_category_name("Politics", locale) },
        Category { id: 25, name: translate_category_name("Art", locale) },
        Category { id: 26, name: translate_category_name("Celebrities", locale) },
        Category { id: 27, name: translate_category_name("Animals", locale) },
        Category { id: 28, name: translate_category_name("Vehicles", locale) },
        Category { id: 29, name: translate_category_name("Entertainment: Comics", locale) },
        Category { id: 30, name: translate_category_name("Science: Gadgets", locale) },
        Category { id: 31, name: translate_category_name("Entertainment: Japanese Anime & Manga", locale) },
        Category { id: 32, name: translate_category_name("Entertainment: Cartoon & Animations", locale) },
    ]
}

pub struct Game {
    pub state: GameState,
    pub api: TriviaApi,
    pub questions: Vec<TriviaQuestion>,
    pub current_question_index: usize,
    pub score: u32,
    pub total_questions: u32,
    pub last_answer_correct: bool,
    pub selected_answer: Option<usize>,
    pub answer_results: Vec<bool>,
    pub selected_category: Option<Category>,
    pub category_index: usize,
    pub locale: Locale,
    pub loading_phase: Option<LoadingPhase>,
    pub translation_done: bool,
}

impl Game {
    pub async fn new(locale: Locale) -> Result<Self> {
        if locale == Locale::Zh && !Translator::is_available() {
            eprintln!("Warning: OPENAI_API_KEY not set, questions will remain in English");
        }

        Ok(Self {
            state: GameState::Menu,
            api: TriviaApi::new(),
            questions: Vec::new(),
            current_question_index: 0,
            score: 0,
            total_questions: 10,
            last_answer_correct: false,
            selected_answer: None,
            answer_results: Vec::new(),
            selected_category: None,
            category_index: 0,
            locale,
            loading_phase: None,
            translation_done: false,
        })
    }

    pub async fn start_game(&mut self) -> Result<()> {
        self.state = GameState::Loading;
        self.loading_phase = Some(LoadingPhase::Fetching);
        self.score = 0;
        self.current_question_index = 0;
        self.answer_results.clear();
        self.translation_done = false;

        let category_id = self.selected_category.as_ref().map(|c| c.id);
        let questions = match self
            .api
            .fetch_questions(self.total_questions, category_id)
            .await
        {
            Ok(q) => q,
            Err(e) => {
                eprintln!("Failed to fetch questions: {}", e);
                vec![TriviaQuestion {
                    category: "General Knowledge".to_string(),
                    r#type: "multiple".to_string(),
                    difficulty: "easy".to_string(),
                    question: "What is 2 + 2?".to_string(),
                    correct_answer: "4".to_string(),
                    incorrect_answers: vec!["2".to_string(), "3".to_string(), "5".to_string()],
                }]
            }
        };

        if self.locale == Locale::Zh && Translator::is_available() {
            debug_log!("Starting translation flow");
            self.loading_phase = Some(LoadingPhase::Translating(None));
            let total = questions.len() as u32;
            debug_log!("Starting translation loop, total: {}", total);
            match Translator::new() {
                Ok(_translator) => {
                    let locale = self.locale;
                    let (tx, mut rx) = tokio::sync::mpsc::channel::<(Vec<TriviaQuestion>, Option<LoadingPhase>, bool)>(1);
                    let tx2 = tx.clone();
                    
                    tokio::spawn(async move {
                        debug_log!("Spawned translation task");
                        let total = questions.len() as u32;
                        let mut handles = Vec::new();
                        
                        for (i, q) in questions.into_iter().enumerate() {
                            let translator = Translator::new().unwrap();
                            let locale = locale;
                            
                            let handle = tokio::spawn(async move {
                                let tq = match translator.translate_question(&q, locale).await {
                                    Ok(tq) => tq,
                                    Err(e) => {
                                        debug_log!("Translation failed for question {}: {}", i + 1, e);
                                        q
                                    }
                                };
                                (i, tq)
                            });
                            handles.push(handle);
                        }
                        
                        let mut translated = Vec::new();
                        for handle in handles {
                            if let Ok((_, tq)) = handle.await {
                                debug_log!("Translation completed: {}/{}", translated.len() + 1, total);
                                translated.push(tq);
                                let phase = LoadingPhase::Translating(Some(TranslationProgress {
                                    current: translated.len() as u32,
                                    total,
                                }));
                                let _ = tx2.send((translated.clone(), Some(phase), false)).await;
                            }
                        }
                        let _ = tx2.send((translated, None, true)).await;
                    });
                    
                    loop {
                        tokio::select! {
                            result = rx.recv() => {
                                debug_log!("Received from channel: {:?}", result);
                                if let Some((q, phase, done)) = result {
                                    self.questions = q;
                                    self.loading_phase = phase;
                                    self.translation_done = done;
                                    debug_log!("Updated game state: translation_done={}", done);
                                    if done {
                                        self.state = GameState::Question;
                                        break;
                                    }
                                }
                            }
                            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                                debug_log!("Timeout, no message received");
                                continue;
                            }
                        }
                    }
                }
                Err(e) => {
                    debug_log!("Failed to create translator: {}", e);
                    self.questions = questions;
                    self.loading_phase = None;
                    self.translation_done = true;
                }
            }
        } else {
            self.questions = questions;
            self.loading_phase = None;
            self.translation_done = true;
            self.state = GameState::Question;
        }

        Ok(())
    }

    pub async fn answer_question(&mut self, answer_index: usize) -> Result<()> {
        if let Some(current_question) = self.current_question() {
            let correct_index = current_question.get_correct_index();
            self.last_answer_correct = answer_index == correct_index;
            self.selected_answer = Some(answer_index);

            if self.last_answer_correct {
                self.score += 1;
            }

            self.answer_results.push(self.last_answer_correct);
            self.next_question().await?;
        }
        Ok(())
    }

    pub async fn next_question(&mut self) -> Result<()> {
        self.current_question_index += 1;
        self.selected_answer = None;

        if self.current_question_index >= self.questions.len() {
            self.state = GameState::GameOver;
        } else {
            self.state = GameState::Question;
        }

        Ok(())
    }

    pub async fn reset_game(&mut self) -> Result<()> {
        self.state = GameState::Menu;
        self.questions.clear();
        self.current_question_index = 0;
        self.score = 0;
        self.selected_answer = None;
        self.answer_results.clear();
        self.selected_category = None;
        self.category_index = 0;
        Ok(())
    }

    pub fn confirm_category(&mut self) {
        let categories = get_categories(self.locale);
        if self.category_index == 0 {
            self.selected_category = None;
        } else {
            let idx = self.category_index - 1;
            if idx < categories.len() {
                self.selected_category = Some(categories[idx].clone());
            }
        }
    }

    pub fn navigate_category(&mut self, down: bool) {
        let max = get_categories(self.locale).len();
        if down {
            self.category_index = (self.category_index + 1) % (max + 1);
        } else {
            if self.category_index == 0 {
                self.category_index = max;
            } else {
                self.category_index -= 1;
            }
        }
    }

    pub fn current_question(&self) -> Option<&TriviaQuestion> {
        self.questions.get(self.current_question_index)
    }

    pub fn progress(&self) -> (usize, usize) {
        (self.current_question_index + 1, self.questions.len())
    }
}
