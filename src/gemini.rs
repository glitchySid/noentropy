use crate::files::OrganizationPlan;
use reqwest::Client;
use serde_json::json;

pub struct GeminiClient {
    api_key: String,
    client: Client,
    base_url: String,
}
impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            base_url: "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent".to_string(),
        }
    }

    /// Takes a list of filenames and asks Gemini to categorize them
    pub async fn organize_files(
        &self,
        filenames: Vec<String>,
    ) -> Result<OrganizationPlan, Box<dyn std::error::Error>> {
        let url = format!("{}?key={}", self.base_url, self.api_key);

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

        // 3. Send
        let res = self.client.post(&url).json(&request_body).send().await?;

        // 4. Parse
        if res.status().is_success() {
            let resp_json: serde_json::Value = res.json().await?;

            // Extract the raw JSON string from Gemini
            let raw_text = resp_json["candidates"][0]["content"]["parts"][0]["text"]
                .as_str()
                .ok_or("Failed to get text from Gemini")?;

            // Deserialize into our Rust Struct
            let plan: OrganizationPlan = serde_json::from_str(raw_text)?;
            Ok(plan)
        } else {
            let err = res.text().await?;
            Err(format!("API Error: {}", err).into())
        }
    }
}
