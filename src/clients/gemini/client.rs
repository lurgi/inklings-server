use super::traits::{Embedder, TextGenerator};
use crate::clients::ClientError;
use serde::{Deserialize, Serialize};

const EMBEDDING_API_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/text-embedding-004:embedContent";
const GENERATION_API_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";

#[derive(Clone)]
pub struct GeminiClient {
    api_key: String,
    client: reqwest::Client,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Serialize)]
struct EmbedRequest {
    content: Content,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embedding: Embedding,
}

#[derive(Deserialize)]
struct Embedding {
    values: Vec<f32>,
}

#[async_trait::async_trait]
impl Embedder for GeminiClient {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, ClientError> {
        let request_body = EmbedRequest {
            content: Content {
                parts: vec![Part {
                    text: text.to_string(),
                }],
            },
        };

        let response = self
            .client
            .post(format!("{}?key={}", EMBEDDING_API_URL, self.api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ClientError::Network(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ClientError::GeminiApi(format!(
                "API request failed with status {}: {}",
                status, error_text
            )));
        }

        let embed_response: EmbedResponse = response.json().await.map_err(|e| {
            ClientError::ParseError(format!("Failed to parse response: {}", e))
        })?;

        Ok(embed_response.embedding.values)
    }

    fn dimension(&self) -> usize {
        768
    }
}

#[derive(Serialize)]
struct GenerateRequest {
    contents: Vec<ContentItem>,
}

#[derive(Serialize)]
struct ContentItem {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct GenerateResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: GeneratedContent,
}

#[derive(Deserialize)]
struct GeneratedContent {
    parts: Vec<GeneratedPart>,
}

#[derive(Deserialize)]
struct GeneratedPart {
    text: String,
}

#[async_trait::async_trait]
impl TextGenerator for GeminiClient {
    async fn generate(
        &self,
        prompt: &str,
        context: Vec<String>,
    ) -> Result<String, ClientError> {
        let mut prompt_text = String::from("다음은 사용자가 과거에 작성한 메모들입니다:\n\n");

        for (i, memo) in context.iter().enumerate() {
            prompt_text.push_str(&format!("메모 {}:\n{}\n\n", i + 1, memo));
        }

        prompt_text.push_str(&format!(
            "위 메모들을 참고하여, 다음 주제에 대한 글쓰기를 도와주세요:\n{}",
            prompt
        ));

        let request_body = GenerateRequest {
            contents: vec![ContentItem {
                parts: vec![Part {
                    text: prompt_text,
                }],
            }],
        };

        let response = self
            .client
            .post(format!("{}?key={}", GENERATION_API_URL, self.api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ClientError::Network(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ClientError::GeminiApi(format!(
                "API request failed with status {}: {}",
                status, error_text
            )));
        }

        let generate_response: GenerateResponse = response.json().await.map_err(|e| {
            ClientError::ParseError(format!("Failed to parse response: {}", e))
        })?;

        let text = generate_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .ok_or_else(|| ClientError::GeminiApi("No response generated".to_string()))?;

        Ok(text)
    }
}
