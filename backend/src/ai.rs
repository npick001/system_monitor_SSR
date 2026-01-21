use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
use reqwest::Client;
use serde_json::json;
use tokio::sync::Mutex;
use std::env;

pub struct Brain {
    embedder: Mutex<TextEmbedding>,
    http_client: Client,
    api_key: String,
}

impl Brain {
    pub fn new() -> Self {
        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::AllMiniLML6V2)
                .with_show_download_progress(true)
        ).expect("Failed to load embedding model");

        let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");

        Self { 
            embedder: Mutex::new(model),
            http_client: Client::new(),
            api_key,
        }
    }

    pub async fn embed(&self, text: &str) -> Vec<f32> {
        let documents = vec![text.to_string()];
        let mut embedder = self.embedder.lock().await;
        let embeddings = embedder.embed(documents, None).unwrap();
        embeddings[0].clone()
    }

    pub async fn ask_gemini(&self, context: String, user_question: String) -> String {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}", 
            self.api_key
        );

        let prompt = format!(
            "You are an AI assistant for Nicholas Pickering's portfolio. 
             Use the following context to answer the user's question. 
             If the answer isn't in the context, say you don't know.
             
             CONTEXT:
             {}
             
             QUESTION:
             {}",
            context, user_question
        );

        let body = json!({
            "contents": [{
                "parts": [{ "text": prompt }]
            }]
        });

        let res = self.http_client.post(&url)
            .json(&body)
            .send()
            .await;

        match res {
            Ok(response) => {
                // Parse the Raw JSON first to inspect it
                let raw_json: serde_json::Value = match response.json().await {
                    Ok(v) => v,
                    Err(_) => return "Failed to parse JSON response from Google.".to_string(),
                };

                // Try to extract the text
                if let Some(candidates) = raw_json.get("candidates") {
                    if let Some(first) = candidates.get(0) {
                        if let Some(content) = first.get("content") {
                            if let Some(parts) = content.get("parts") {
                                if let Some(first_part) = parts.get(0) {
                                    if let Some(text) = first_part.get("text") {
                                        return text.as_str().unwrap_or("").to_string();
                                    }
                                }
                            }
                        }
                    }
                }

                // --- DEBUGGING: If we get here, the structure was wrong. Print why! ---
                println!("⚠️ GEMINI ERROR: {:?}", raw_json);
                
                // Check for common API errors
                if let Some(error) = raw_json.get("error") {
                    if let Some(msg) = error.get("message") {
                        return format!("Gemini API Error: {}", msg);
                    }
                }

                "Error parsing Gemini response (Check Docker Logs for raw JSON).".to_string()
            }
            Err(e) => format!("Error calling Gemini: {}", e),
        }
    }
}