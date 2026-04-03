use anyhow::Result;
use crate::api::{Category, TriviaApi, TriviaQuestion};

#[derive(Debug, Clone)]
pub enum GameState {
    Menu,
    SelectCategory,
    Loading,
    Question,
    GameOver,
}

pub fn get_categories() -> Vec<Category> {
    vec![
        Category { id: 9, name: "General Knowledge".to_string() },
        Category { id: 10, name: "Books".to_string() },
        Category { id: 11, name: "Film".to_string() },
        Category { id: 12, name: "Music".to_string() },
        Category { id: 17, name: "Science & Nature".to_string() },
        Category { id: 18, name: "Computers".to_string() },
        Category { id: 19, name: "Mathematics".to_string() },
        Category { id: 21, name: "Sports".to_string() },
        Category { id: 22, name: "Geography".to_string() },
        Category { id: 23, name: "History".to_string() },
        Category { id: 27, name: "Animals".to_string() },
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
}

impl Game {
    pub async fn new() -> Result<Self> {
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
        })
    }

    pub async fn start_game(&mut self) -> Result<()> {
        self.state = GameState::Loading;
        self.score = 0;
        self.current_question_index = 0;
        self.answer_results.clear(); // Clear previous results
        
        // Fetch questions from API
        let category_id = self.selected_category.as_ref().map(|c| c.id);
        match self.api.fetch_questions(self.total_questions, category_id).await {
            Ok(questions) => {
                self.questions = questions;
                self.state = GameState::Question;
            }
            Err(e) => {
                // For demo purposes, we'll create a fallback question
                eprintln!("Failed to fetch questions: {}", e);
                self.questions = vec![TriviaQuestion {
                    category: "General Knowledge".to_string(),
                    r#type: "multiple".to_string(),
                    difficulty: "easy".to_string(),
                    question: "What is 2 + 2?".to_string(),
                    correct_answer: "4".to_string(),
                    incorrect_answers: vec!["2".to_string(), "3".to_string(), "5".to_string()],
                }];
                self.state = GameState::Question;
            }
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
            
            // Track the result of this answer
            self.answer_results.push(self.last_answer_correct);
            
            // Directly go to next question without showing result one by one
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
        let categories = get_categories();
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
        let max = get_categories().len();
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
