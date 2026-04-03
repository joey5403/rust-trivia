use crate::api::TriviaQuestion;
use crate::locale::Locale;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

pub struct Translator {
    api_key: String,
    model: String,
    base_url: String,
    client: Client,
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

impl Translator {
    pub fn new() -> Result<Self> {
        let api_key =
            env::var("OPENAI_API_KEY").context("OPENAI_API_KEY environment variable not set")?;

        let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());

        let base_url =
            env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

        Ok(Self {
            api_key,
            model,
            base_url,
            client: Client::new(),
        })
    }

    pub fn is_available() -> bool {
        env::var("OPENAI_API_KEY").is_ok()
    }

    pub async fn translate_text(&self, text: &str, to: Locale) -> Result<String> {
        if to == Locale::En {
            return Ok(text.to_string());
        }

        let target_lang = match to {
            Locale::Zh => "Chinese",
            Locale::En => "English",
        };

        let prompt = format!("Translate to {}: {}", target_lang, text);

        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send translation request")?;

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse translation response")?;

        let content = chat_response
            .choices
            .first()
            .context("No translation response received")?
            .message
            .content
            .trim()
            .to_string();

        if content.is_empty() {
            anyhow::bail!("Translation returned empty response");
        }

        Ok(content)
    }

    pub async fn translate_question(
        &self,
        question: &TriviaQuestion,
        locale: Locale,
    ) -> Result<TriviaQuestion> {
        if locale == Locale::En {
            return Ok(question.clone());
        }

        let translated_question = self.translate_text(&question.question, locale).await?;
        let translated_category = self.translate_text(&question.category, locale).await?;
        let translated_difficulty = self
            .translate_difficulty(&question.difficulty, locale)
            .await?;

        let mut translated_incorrect = Vec::new();
        for answer in &question.incorrect_answers {
            let translated = self.translate_text(answer, locale).await?;
            translated_incorrect.push(translated);
        }

        let translated_correct = self
            .translate_text(&question.correct_answer, locale)
            .await?;

        Ok(TriviaQuestion {
            category: translated_category,
            difficulty: translated_difficulty,
            question: translated_question,
            correct_answer: translated_correct,
            incorrect_answers: translated_incorrect,
            r#type: question.r#type.clone(),
        })
    }

    async fn translate_difficulty(&self, difficulty: &str, locale: Locale) -> Result<String> {
        if locale == Locale::En {
            return Ok(difficulty.to_string());
        }

        match difficulty.to_lowercase().as_str() {
            "easy" => Ok("简单".to_string()),
            "medium" => Ok("中等".to_string()),
            "hard" => Ok("困难".to_string()),
            _ => self.translate_text(difficulty, locale).await,
        }
    }
}
