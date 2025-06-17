use anyhow::Result;
use crate::api::{TriviaApi, TriviaQuestion};

#[derive(Debug, Clone)]
pub enum GameState {
    Menu,
    Loading,
    Question,
    ShowResult,
    GameOver,
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
    pub answer_results: Vec<bool>, // Track correct/incorrect for each question
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
            answer_results: Vec::new(), // Initialize with an empty vector
        })
    }

    pub async fn start_game(&mut self) -> Result<()> {
        self.state = GameState::Loading;
        self.score = 0;
        self.current_question_index = 0;
        self.answer_results.clear(); // Clear previous results
        
        // Fetch questions from API
        match self.api.fetch_questions(self.total_questions).await {
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
            
            self.state = GameState::ShowResult;
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
        self.answer_results.clear(); // Clear the answer results
        Ok(())
    }

    pub fn current_question(&self) -> Option<&TriviaQuestion> {
        self.questions.get(self.current_question_index)
    }

    pub fn progress(&self) -> (usize, usize) {
        (self.current_question_index + 1, self.questions.len())
    }
}
