use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
pub struct Candidate {
    pub content: Content,
}

#[derive(Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Deserialize)]
pub struct FileCategoryResponse {
    pub filename: String,
    pub category: String,
}

#[derive(Deserialize)]
pub struct OrganizationPlanResponse {
    pub files: Vec<FileCategoryResponse>,
}
