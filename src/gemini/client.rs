use crate::gemini::errors::GeminiError;
use crate::gemini::prompt::PromptBuilder;
use crate::gemini::types::{GeminiResponse, OrganizationPlanResponse};
use crate::models::OrganizationPlan;
use crate::storage::Cache;
use reqwest::Client;
use serde_json::json;
use std::path::Path;
use std::time::Duration;

const DEFAULT_MODEL: &str = "gemini-3-flash-preview";
const DEFAULT_TIMEOUT_SECS: u64 = 120;
const MAX_RETRIES: u32 = 3;
const BATCH_SIZE: usize = 50;

pub struct GeminiClient {
    api_key: String,
    client: Client,
    base_url: String,
    #[allow(dead_code)]
    model: String,
    #[allow(dead_code)]
    timeout: Duration,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self::with_model(api_key, DEFAULT_MODEL.to_string())
    }

    pub fn with_model(api_key: String, model: String) -> Self {
        let timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECS);
        let client = Self::build_client(timeout);
        let base_url = Self::build_base_url(&model);

        Self {
            api_key,
            client,
            base_url,
            model,
            timeout,
        }
    }

    fn build_client(timeout: Duration) -> Client {
        Client::builder()
            .timeout(timeout)
            .build()
            .unwrap_or_default()
    }

    fn build_base_url(model: &str) -> String {
        format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            model
        )
    }

    pub async fn organize_files(
        &self,
        filenames: Vec<String>,
    ) -> Result<OrganizationPlan, GeminiError> {
        self.organize_files_with_cache(filenames, None, None).await
    }

    pub async fn organize_files_with_cache(
        &self,
        filenames: Vec<String>,
        mut cache: Option<&mut Cache>,
        base_path: Option<&Path>,
    ) -> Result<OrganizationPlan, GeminiError> {
        let url = self.build_url();

        if let (Some(cache), Some(base_path)) = (cache.as_ref(), base_path)
            && let Some(cached_response) = cache.get_cached_response(&filenames, base_path)
        {
            return Ok(cached_response);
        }

        let prompt = PromptBuilder::new(filenames.clone()).build_categorization_prompt();
        let request_body = self.build_categorization_request(&prompt);

        let res = self.send_request_with_retry(&url, &request_body).await?;
        let plan = self.parse_categorization_response(res).await?;

        if let (Some(cache), Some(base_path)) = (cache.as_mut(), base_path) {
            cache.cache_response(&filenames, plan.clone(), base_path);
        }

        Ok(plan)
    }

    /// Organizes files in batches to handle large file lists efficiently.
    ///
    /// When the number of files exceeds BATCH_SIZE, splits them into smaller
    /// chunks to avoid API timeout and payload size issues. Each batch is
    /// processed sequentially with progress feedback.
    ///
    /// # Arguments
    /// * `filenames` - Vector of filenames to organize
    /// * `cache` - Optional cache for storing/retrieving results
    /// * `base_path` - Optional base path for cache keys
    ///
    /// # Returns
    /// A combined `OrganizationPlan` with all files categorized
    pub async fn organize_files_in_batches(
        &self,
        filenames: Vec<String>,
        mut cache: Option<&mut Cache>,
        base_path: Option<&Path>,
    ) -> Result<OrganizationPlan, GeminiError> {
        // No batching needed for small file lists
        if filenames.len() <= BATCH_SIZE {
            return self
                .organize_files_with_cache(filenames, cache, base_path)
                .await;
        }

        let total_files = filenames.len();
        let batches: Vec<Vec<String>> = filenames
            .chunks(BATCH_SIZE)
            .map(|chunk| chunk.to_vec())
            .collect();
        let total_batches = batches.len();

        println!(
            "Processing {} files in {} batches...",
            total_files, total_batches
        );

        let mut all_files = Vec::with_capacity(total_files);

        for (batch_index, batch) in batches.into_iter().enumerate() {
            let batch_num = batch_index + 1;
            println!(
                "Processing batch {}/{} ({} files)...",
                batch_num,
                total_batches,
                batch.len()
            );

            let plan = self
                .organize_files_with_cache(batch, cache.as_deref_mut(), base_path)
                .await?;

            all_files.extend(plan.files);
        }

        Ok(OrganizationPlan { files: all_files })
    }

    fn build_url(&self) -> String {
        format!("{}?key={}", self.base_url, self.api_key)
    }

    fn build_categorization_request(&self, prompt: &str) -> serde_json::Value {
        json!({
            "contents": [{ "parts": [{ "text": prompt }] }],
            "generationConfig": { "response_mime_type": "application/json" }
        })
    }

    async fn parse_categorization_response(
        &self,
        res: reqwest::Response,
    ) -> Result<OrganizationPlan, GeminiError> {
        if !res.status().is_success() {
            return Err(GeminiError::from_response(res).await);
        }

        let gemini_response: GeminiResponse =
            res.json().await.map_err(GeminiError::NetworkError)?;

        let raw_text = self.extract_text_from_response(&gemini_response)?;
        let plan_response: OrganizationPlanResponse = serde_json::from_str(&raw_text)?;

        Ok(plan_response.to_organization_plan())
    }

    fn extract_text_from_response(&self, response: &GeminiResponse) -> Result<String, GeminiError> {
        response
            .candidates
            .first()
            .ok_or_else(|| GeminiError::InvalidResponse("No candidates in response".to_string()))?
            .content
            .parts
            .first()
            .ok_or_else(|| GeminiError::InvalidResponse("No parts in content".to_string()))
            .map(|p| p.text.clone())
    }

    async fn send_request_with_retry(
        &self,
        url: &str,
        request_body: &serde_json::Value,
    ) -> Result<reqwest::Response, GeminiError> {
        let mut attempts = 0;
        let mut base_delay = Duration::from_secs(2);

        loop {
            attempts += 1;

            match self.client.post(url).json(request_body).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        return Ok(response);
                    }

                    let error = GeminiError::from_response(response).await;

                    if error.is_retryable() && attempts < MAX_RETRIES {
                        let delay = error.retry_delay().unwrap_or(base_delay);
                        self.print_retry_message(&error, delay, attempts);
                        tokio::time::sleep(delay).await;
                        base_delay *= 2;
                        continue;
                    }

                    return Err(error);
                }
                Err(e) => {
                    if attempts < MAX_RETRIES {
                        self.print_network_retry(&e, base_delay, attempts);
                        tokio::time::sleep(base_delay).await;
                        base_delay *= 2;
                        continue;
                    }
                    return Err(GeminiError::NetworkError(e));
                }
            }
        }
    }

    fn print_retry_message(&self, error: &GeminiError, delay: Duration, attempt: u32) {
        println!(
            "API Error: {}. Retrying in {} seconds (attempt {}/{})",
            error,
            delay.as_secs(),
            attempt,
            MAX_RETRIES
        );
    }

    fn print_network_retry(&self, error: &reqwest::Error, delay: Duration, attempt: u32) {
        println!(
            "Network error: {}. Retrying in {} seconds (attempt {}/{})",
            error,
            delay.as_secs(),
            attempt,
            MAX_RETRIES
        );
    }

    pub async fn get_ai_sub_category(
        &self,
        filename: &str,
        parent_category: &str,
        content: &str,
    ) -> String {
        let url = self.build_url();
        let prompt = PromptBuilder::build_subcategory_prompt(filename, parent_category, content);
        let request_body = self.build_subcategory_request(&prompt);

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

        self.parse_subcategory_response(res, filename).await
    }

    fn build_subcategory_request(&self, prompt: &str) -> serde_json::Value {
        json!({
            "contents": [{ "parts": [{ "text": prompt }] }]
        })
    }

    async fn parse_subcategory_response(&self, res: reqwest::Response, filename: &str) -> String {
        if !res.status().is_success() {
            eprintln!(
                "Warning: API returned error for {}: {}",
                filename,
                res.status()
            );
            return "General".to_string();
        }

        let gemini_response: GeminiResponse = match res.json().await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Warning: Failed to parse response for {}: {}", filename, e);
                return "General".to_string();
            }
        };

        self.extract_subcategory_from_response(&gemini_response, filename)
    }

    fn extract_subcategory_from_response(
        &self,
        response: &GeminiResponse,
        _filename: &str,
    ) -> String {
        match self.extract_text_from_response(response) {
            Ok(text) => {
                let sub_category = text.trim();
                if sub_category.is_empty() {
                    "General".to_string()
                } else {
                    sub_category.to_string()
                }
            }
            Err(_) => "General".to_string(),
        }
    }
}
