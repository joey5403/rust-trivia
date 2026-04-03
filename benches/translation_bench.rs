use rust_trivia::api::TriviaQuestion;
use rust_trivia::locale::Locale;
use rust_trivia::translation::Translator;
use std::time::Instant;

#[tokio::main]
async fn main() {
    // Check if API key is set
    if !Translator::is_available() {
        eprintln!("OPENAI_API_KEY not set, skipping benchmark");
        return;
    }

    let translator = Translator::new().expect("Failed to create translator");
    let locale = Locale::Zh;

    // Sample question
    let question = TriviaQuestion {
        category: "Science & Nature".to_string(),
        r#type: "multiple".to_string(),
        difficulty: "medium".to_string(),
        question: "What is the chemical symbol for gold?".to_string(),
        correct_answer: "Au".to_string(),
        incorrect_answers: vec![
            "Ag".to_string(),
            "Fe".to_string(),
            "Cu".to_string(),
        ],
    };

    println!("=== Translation Speed Benchmark ===\n");
    println!("Question: {}", question.question);
    println!("Target locale: {:?}\n", locale);

    // Warm up
    println!("Warming up...");
    let _ = translator.translate_text("test", locale).await;

    // Benchmark translate_question (full question translation)
    println!("\n--- Full Question Translation ---");
    let start = Instant::now();
    let result = translator.translate_question(&question, locale).await;
    let elapsed = start.elapsed();

    match result {
        Ok(tq) => {
            println!("Translated question: {}", tq.question);
            println!("Translated category: {}", tq.category);
            println!("Translated difficulty: {}", tq.difficulty);
            println!("Time: {:.2}s ({}ms)", elapsed.as_secs_f64(), elapsed.as_millis());
        }
        Err(e) => {
            eprintln!("Translation failed: {}", e);
        }
    }

    // Benchmark individual field translations
    println!("\n--- Individual Field Translation ---");
    let fields = [
        ("Question", question.question.as_str()),
        ("Category", question.category.as_str()),
        ("Correct Answer", question.correct_answer.as_str()),
        ("Incorrect 1", question.incorrect_answers[0].as_str()),
    ];

    let mut total_time = std::time::Duration::from_secs(0);
    for (name, text) in fields {
        let start = Instant::now();
        let result = translator.translate_text(text, locale).await;
        let elapsed = start.elapsed();
        total_time += elapsed;

        match result {
            Ok(translated) => {
                println!("{}: {:.2}ms -> {}", name, elapsed.as_millis() as f64, translated);
            }
            Err(e) => {
                eprintln!("{} failed: {}", name, e);
            }
        }
    }

    println!("\nTotal time for {} fields: {:.2}s", fields.len(), total_time.as_secs_f64());
    println!("Average per field: {:.2}ms", total_time.as_millis() as f64 / fields.len() as f64);
}