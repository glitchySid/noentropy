use reqwest::Response;
use serde::Deserialize;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeminiError {
    #[error("API rate limit exceeded. Retry after {retry_after} seconds")]
    RateLimitExceeded { retry_after: u32 },

    #[error("Quota exceeded. Usage limit reached: {limit}")]
    QuotaExceeded { limit: String },

    #[error("Model '{model}' not found or unavailable")]
    ModelNotFound { model: String },

    #[error("Invalid API key. Please check your GEMINI_API_KEY")]
    InvalidApiKey,

    #[error("Content policy violation: {reason}")]
    ContentPolicyViolation { reason: String },

    #[error("Invalid request: {details}")]
    InvalidRequest { details: String },

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    #[error("API error (HTTP {status}): {message}")]
    ApiError { status: u16, message: String },

    #[error("Service temporarily unavailable: {reason}")]
    ServiceUnavailable { reason: String },

    #[error("Request timeout after {seconds} seconds")]
    Timeout { seconds: u64 },

    #[error("JSON serialization/deserialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Internal server error: {details}")]
    InternalError { details: String },
}

#[derive(Debug, Deserialize)]
struct GeminiErrorResponse {
    error: GeminiErrorDetail,
}

#[derive(Debug, Deserialize)]
struct GeminiErrorDetail {
    #[allow(dead_code)]
    code: i32,
    message: String,
    status: String,
    #[serde(default)]
    details: Vec<GeminiErrorDetailInfo>,
}

#[derive(Debug, Deserialize)]
struct GeminiErrorDetailInfo {
    #[serde(rename = "@type")]
    #[allow(dead_code)]
    error_type: String,
    #[serde(rename = "retryDelay")]
    retry_delay: Option<String>,
    quota_limit: Option<String>,
    #[allow(dead_code)]
    quota_metro: Option<String>,
}

impl GeminiError {
    /// Parse HTTP response and convert to appropriate GeminiError
    pub async fn from_response(response: Response) -> Self {
        let status = response.status();

        let error_text = match response.text().await {
            Ok(text) => text,
            Err(e) => {
                return GeminiError::NetworkError(e);
            }
        };

        if let Ok(gemini_error) = serde_json::from_str::<GeminiErrorResponse>(&error_text) {
            return Self::from_gemini_error(gemini_error.error, status.as_u16());
        }

        Self::from_status_code(status, &error_text)
    }

    fn from_gemini_error(error_detail: GeminiErrorDetail, status: u16) -> Self {
        let details = error_detail.details;

        match error_detail.status.as_str() {
            "RESOURCE_EXHAUSTED" => {
                if let Some(retry_info) = details.iter().find(|d| d.retry_delay.is_some())
                    && let Some(retry_delay) = &retry_info.retry_delay
                    && let Ok(seconds) = retry_delay.parse::<u32>()
                {
                    return GeminiError::RateLimitExceeded {
                        retry_after: seconds,
                    };
                }

                if let Some(quota_info) = details.iter().find(|d| d.quota_limit.is_some()) {
                    let limit = quota_info.quota_limit.as_deref().unwrap_or("unknown");
                    return GeminiError::QuotaExceeded {
                        limit: limit.to_string(),
                    };
                }

                GeminiError::QuotaExceeded {
                    limit: "usage limit".to_string(),
                }
            }
            "NOT_FOUND" => {
                // Extract model name from message if possible
                let model = extract_model_name(&error_detail.message);
                GeminiError::ModelNotFound { model }
            }
            "UNAUTHENTICATED" => GeminiError::InvalidApiKey,
            "PERMISSION_DENIED" => {
                if error_detail.message.to_lowercase().contains("policy") {
                    GeminiError::ContentPolicyViolation {
                        reason: error_detail.message,
                    }
                } else {
                    GeminiError::InvalidRequest {
                        details: error_detail.message,
                    }
                }
            }
            "INVALID_ARGUMENT" => GeminiError::InvalidRequest {
                details: error_detail.message,
            },
            "UNAVAILABLE" => GeminiError::ServiceUnavailable {
                reason: error_detail.message,
            },
            "DEADLINE_EXCEEDED" => GeminiError::Timeout { seconds: 60 },
            "INTERNAL" => GeminiError::InternalError {
                details: error_detail.message,
            },
            _ => GeminiError::ApiError {
                status,
                message: error_detail.message,
            },
        }
    }

    fn from_status_code(status: reqwest::StatusCode, error_text: &str) -> Self {
        match status.as_u16() {
            400 => GeminiError::InvalidRequest {
                details: error_text.to_string(),
            },
            401 => GeminiError::InvalidApiKey,
            403 => GeminiError::ContentPolicyViolation {
                reason: error_text.to_string(),
            },
            404 => GeminiError::ModelNotFound {
                model: "unknown".to_string(),
            },
            429 => GeminiError::RateLimitExceeded { retry_after: 60 },
            500 => GeminiError::InternalError {
                details: error_text.to_string(),
            },
            502..=504 => GeminiError::ServiceUnavailable {
                reason: error_text.to_string(),
            },
            _ => GeminiError::ApiError {
                status: status.as_u16(),
                message: error_text.to_string(),
            },
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            GeminiError::RateLimitExceeded { .. }
                | GeminiError::ServiceUnavailable { .. }
                | GeminiError::Timeout { .. }
                | GeminiError::NetworkError(_)
                | GeminiError::InternalError { .. }
        )
    }

    /// Get retry delay for retryable errors
    pub fn retry_delay(&self) -> Option<Duration> {
        match self {
            GeminiError::RateLimitExceeded { retry_after } => {
                Some(Duration::from_secs(*retry_after as u64))
            }
            GeminiError::ServiceUnavailable { .. } => Some(Duration::from_secs(30)),
            GeminiError::NetworkError(_) => Some(Duration::from_secs(5)),
            GeminiError::Timeout { .. } => Some(Duration::from_secs(10)),
            GeminiError::InternalError { .. } => Some(Duration::from_secs(15)),
            _ => None,
        }
    }
}

fn extract_model_name(message: &str) -> String {
    // Try to extract model name from error message
    // Example: "Model 'gemini-1.5-flash' not found"
    if let Some(start) = message.find('\'')
        && let Some(end) = message[start + 1..].find('\'')
    {
        return message[start + 1..start + 1 + end].to_string();
    }
    "unknown".to_string()
}
