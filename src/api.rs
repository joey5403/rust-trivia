use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TriviaResponse {
    pub response_code: u32,
    pub results: Vec<TriviaQuestion>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TriviaQuestion {
    pub category: String,
    pub r#type: String,
    pub difficulty: String,
    pub question: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>,
}

impl TriviaQuestion {
    pub fn get_all_answers(&self) -> Vec<String> {
        let mut answers = self.incorrect_answers.clone();
        answers.push(self.correct_answer.clone());
        answers.sort();
        answers
    }

    pub fn get_correct_index(&self) -> usize {
        let all_answers = self.get_all_answers();
        all_answers
            .iter()
            .position(|answer| answer == &self.correct_answer)
            .unwrap_or(0)
    }
}

pub struct TriviaApi {
    client: Client,
}

impl TriviaApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_questions(&self, amount: u32) -> Result<Vec<TriviaQuestion>> {
        let url = format!(
            "https://opentdb.com/api.php?amount={}&type=multiple",
            amount
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<TriviaResponse>()
            .await?;

        if response.response_code == 0 {
            Ok(response.results)
        } else {
            Err(anyhow::anyhow!(
                "API returned error code: {}",
                response.response_code
            ))
        }
    }
}
