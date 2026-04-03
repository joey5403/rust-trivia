use crate::api::TriviaQuestion;
use crate::locale::Locale;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

const CATEGORY_ZH: &[(&str, &str)] = &[
    ("General Knowledge", "常识"),
    ("Entertainment: Books", "娱乐: 书籍"),
    ("Entertainment: Film", "娱乐: 电影"),
    ("Entertainment: Music", "娱乐: 音乐"),
    ("Entertainment: Musicals & Theatres", "娱乐: 音乐剧"),
    ("Entertainment: Television", "娱乐: 电视"),
    ("Entertainment: Video Games", "娱乐: 电子游戏"),
    ("Entertainment: Board Games", "娱乐: 桌游"),
    ("Science & Nature", "科学自然"),
    ("Science: Computers", "计算机科学"),
    ("Science: Mathematics", "数学"),
    ("Mythology", "神话"),
    ("Sports", "体育"),
    ("Geography", "地理"),
    ("History", "历史"),
    ("Politics", "政治"),
    ("Art", "艺术"),
    ("Celebrities", "名人"),
    ("Animals", "动物"),
    ("Vehicles", "交通工具"),
    ("Entertainment: Comics", "娱乐: 漫画"),
    ("Science: Gadgets", "科技 gadgets"),
    ("Entertainment: Japanese Anime & Manga", "娱乐: 日漫"),
    ("Entertainment: Cartoon & Animations", "娱乐: 动画"),
];

pub fn translate_category_name(category: &str, locale: Locale) -> String {
    match locale {
        Locale::Zh => CATEGORY_ZH
            .iter()
            .find(|(en, _)| category.to_lowercase().contains(&en.to_lowercase()))
            .map(|(_, zh)| *zh)
            .unwrap_or(category)
            .to_string(),
        Locale::En => category.to_string(),
    }
}

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
    #[serde(skip_serializing_if = "Option::is_none")]
    thinking: Option<ChatThinking>,
}

#[derive(Serialize)]
struct ChatThinking {
    #[serde(rename = "type")]
    thinking_type: String,
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

    pub async fn translate_question(
        &self,
        question: &TriviaQuestion,
        locale: Locale,
    ) -> Result<TriviaQuestion> {
        if locale == Locale::En {
            return Ok(question.clone());
        }

        let target_lang = match locale {
            Locale::Zh => "Chinese",
            Locale::En => "English",
        };

        let prompt = format!(
            r#"Translate the following trivia question to {target_lang}. Keep the same format:

Q: {question}
CATEGORY: {category}
DIFFICULTY: {difficulty}
A: {correct}
B: {incorrect1}
C: {incorrect2}
D: {incorrect3}"#,
            question = question.question,
            category = question.category,
            difficulty = question.difficulty,
            correct = question.correct_answer,
            incorrect1 = question.incorrect_answers.get(0).cloned().unwrap_or_default(),
            incorrect2 = question.incorrect_answers.get(1).cloned().unwrap_or_default(),
            incorrect3 = question.incorrect_answers.get(2).cloned().unwrap_or_default(),
        );

        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            thinking: Some(ChatThinking {
                thinking_type: "disabled".to_string(),
            }),
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

        self.parse_translated_question(&content, question)
    }

    fn parse_translated_question(
        &self,
        response: &str,
        original: &TriviaQuestion,
    ) -> Result<TriviaQuestion> {
        let lines = response.lines();
        let mut translated_question = String::new();
        let mut translated_category = String::new();
        let mut translated_difficulty = String::new();
        let mut translated_correct = String::new();
        let mut translated_incorrect = Vec::new();

        for line in lines {
            let line = line.trim();
            if line.starts_with("Q:") || line.starts_with("Q：") {
                translated_question = line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string();
            } else if line.starts_with("CATEGORY:") || line.starts_with("CATEGORY：") {
                translated_category = line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string();
            } else if line.starts_with("DIFFICULTY:") || line.starts_with("DIFFICULTY：") {
                let diff = line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string();
                translated_difficulty = match diff.to_lowercase().as_str() {
                    "easy" | "简单" => "简单".to_string(),
                    "medium" | "中等" => "中等".to_string(),
                    "hard" | "困难" => "困难".to_string(),
                    _ => diff,
                };
            } else if line.starts_with("A:") || line.starts_with("A：") {
                translated_correct = line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string();
            } else if line.starts_with("B:") || line.starts_with("B：") {
                translated_incorrect.push(line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string());
            } else if line.starts_with("C:") || line.starts_with("C：") {
                translated_incorrect.push(line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string());
            } else if line.starts_with("D:") || line.starts_with("D：") {
                translated_incorrect.push(line.splitn(2, ':').nth(1).unwrap_or("").trim().to_string());
            }
        }

        if translated_question.is_empty() {
            anyhow::bail!("Failed to parse translated question from response");
        }
        if translated_category.is_empty() {
            translated_category = original.category.clone();
        }
        if translated_difficulty.is_empty() {
            translated_difficulty = original.difficulty.clone();
        }
        if translated_correct.is_empty() {
            translated_correct = original.correct_answer.clone();
        }
        while translated_incorrect.len() < original.incorrect_answers.len() {
            translated_incorrect.push(original.incorrect_answers.get(translated_incorrect.len()).cloned().unwrap_or_default());
        }

        Ok(TriviaQuestion {
            category: translated_category,
            difficulty: translated_difficulty,
            question: translated_question,
            correct_answer: translated_correct,
            incorrect_answers: translated_incorrect,
            r#type: original.r#type.clone(),
        })
    }
}
