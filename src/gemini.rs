use crate::cache::Cache;
use crate::files::{FileCategory, OrganizationPlan};
use crate::gemini_errors::GeminiError;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::path::Path;
use std::time::Duration;

#[derive(Deserialize, Default)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct FileCategoryResponse {
    filename: String,
    category: String,
}

#[derive(Deserialize)]
struct OrganizationPlanResponse {
    files: Vec<FileCategoryResponse>,
}

pub struct GeminiClient {
    api_key: String,
    client: Client,
    base_url: String,
    model: String,
    timeout: Duration,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self::with_model(api_key, "gemini-3-flash-preview".to_string())
    }

    pub fn with_model(api_key: String, model: String) -> Self {
        Self {
            api_key,
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            base_url: format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
                model
            ),
            model,
            timeout: Duration::from_secs(30),
        }
    }

    /// Takes a list of filenames and asks Gemini to categorize them
    pub async fn organize_files(
        &self,
        filenames: Vec<String>,
    ) -> Result<OrganizationPlan, GeminiError> {
        self.organize_files_with_cache(filenames, None, None).await
    }

    /// Takes a list of filenames and asks Gemini to categorize them with caching support
    pub async fn organize_files_with_cache(
        &self,
        filenames: Vec<String>,
        mut cache: Option<&mut Cache>,
        base_path: Option<&Path>,
    ) -> Result<OrganizationPlan, GeminiError> {
        let url = format!("{}?key={}", self.base_url, self.api_key);

        // Check cache first if available
        if let (Some(cache_ref), Some(base_path)) = (cache.as_ref(), base_path)
            && let Some(cached_response) = cache_ref.get_cached_response(&filenames, base_path)
        {
            return Ok(cached_response);
        }

        // 1. Construct the Prompt
        let file_list_str = filenames.join(", ");
        let prompt = format!(
            "I have these files in my Downloads folder: [{}]. \
            Categorize them into these folders: 'Images', 'Documents', 'Installers', 'Music', 'Archives', 'Code', 'Misc'. \
            Return ONLY a JSON object with this structure: {{ 'files': [ {{ 'filename': 'name', 'category': 'folder' }} ] }}",
            file_list_str
        );

        // 2. Build Request with JSON Mode enforced
        let request_body = json!({
            "contents": [{
                "parts": [{ "text": prompt }]
            }],
            "generationConfig": {
                "response_mime_type": "application/json"
            }
        });

        // 3. Send with retry logic
        let res = self.send_request_with_retry(&url, &request_body).await?;

        // 4. Parse
        if res.status().is_success() {
            let gemini_response: GeminiResponse =
                res.json().await.map_err(GeminiError::NetworkError)?;

            // Extract raw JSON string from Gemini using proper structs
            let raw_text = &gemini_response
                .candidates
                .first()
                .ok_or_else(|| {
                    GeminiError::InvalidResponse("No candidates in response".to_string())
                })?
                .content
                .parts
                .first()
                .ok_or_else(|| GeminiError::InvalidResponse("No parts in content".to_string()))?
                .text;

            // Deserialize into our temporary response struct
            let plan_response: OrganizationPlanResponse = serde_json::from_str(raw_text)?;

            // Manually map to the final OrganizationPlan
            let plan = OrganizationPlan {
                files: plan_response
                    .files
                    .into_iter()
                    .map(|f| FileCategory {
                        filename: f.filename,
                        category: f.category,
                        sub_category: String::new(), // Initialize with empty sub_category
                    })
                    .collect(),
            };

            // Cache the response if cache is available
            if let (Some(cache), Some(base_path)) = (cache.as_mut(), base_path) {
                cache.cache_response(&filenames, plan.clone(), base_path);
            }

            Ok(plan)
        } else {
            Err(GeminiError::from_response(res).await)
        }
    }

    /// Send request with retry logic for retryable errors
    async fn send_request_with_retry(
        &self,
        url: &str,
        request_body: &serde_json::Value,
    ) -> Result<reqwest::Response, GeminiError> {
        let mut attempts = 0;
        let max_attempts = 3;
        let mut base_delay = Duration::from_secs(2);

        loop {
            attempts += 1;

            match self.client.post(url).json(request_body).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        return Ok(response);
                    }

                    let error = GeminiError::from_response(response).await;

                    if error.is_retryable() && attempts < max_attempts {
                        let delay = error.retry_delay().unwrap_or(base_delay);
                        println!(
                            "API Error: {}. Retrying in {} seconds (attempt {}/{})",
                            error,
                            delay.as_secs(),
                            attempts,
                            max_attempts
                        );
                        tokio::time::sleep(delay).await;
                        base_delay *= 2;
                        continue;
                    }

                    return Err(error);
                }
                Err(e) => {
                    if attempts < max_attempts {
                        println!(
                            "Network error: {}. Retrying in {} seconds (attempt {}/{})",
                            e,
                            base_delay.as_secs(),
                            attempts,
                            max_attempts
                        );
                        tokio::time::sleep(base_delay).await;
                        base_delay *= 2;
                        continue;
                    }
                    return Err(GeminiError::NetworkError(e));
                }
            }
        }
    }

    pub async fn get_ai_sub_category(
        &self,
        filename: &str,
        parent_category: &str,
        content: &str,
    ) -> String {
        let url = format!("{}?key={}", self.base_url, self.api_key);

        let prompt = format!(
            "I have a file named '{}' inside the '{}' folder. Here is the first 1000 characters of the content:\n---\n{}\n---\nBased on this, suggest a single short sub-folder name (e.g., 'Invoices', 'Notes', 'Config'). Return ONLY the name of the sub-folder. Do not use markdown or explanations.",
            filename, parent_category, content
        );

        let request_body = json!({
            "contents": [{
                "parts": [{ "text": prompt }]
            }]
        });

        let res = match self.client.post(&url).json(&request_body).send().await {
            Ok(res) => res,
            Err(e) => {
                eprintln!(
                    "Warning: Failed to get sub-category for {}: {}",
                    filename, e
                );
                return "General".to_string();
            }
        };

        if res.status().is_success() {
            let gemini_response: GeminiResponse = match res.json().await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Warning: Failed to parse response for {}: {}", filename, e);
                    return "General".to_string();
                }
            };

            let sub_category = gemini_response
                .candidates
                .first()
                .and_then(|c| c.content.parts.first())
                .map(|p| p.text.trim())
                .unwrap_or("General")
                .to_string();

            if sub_category.is_empty() {
                "General".to_string()
            } else {
                sub_category
            }
        } else {
            eprintln!(
                "Warning: API returned error for {}: {}",
                filename,
                res.status()
            );
            "General".to_string()
        }
    }
}
