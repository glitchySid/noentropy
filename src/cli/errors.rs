use colored::*;

pub fn handle_gemini_error(error: crate::gemini::GeminiError) {
    match error {
        crate::gemini::GeminiError::RateLimitExceeded { retry_after } => {
            println!(
                "{} API rate limit exceeded. Please wait {} seconds before trying again.",
                "ERROR:".red(),
                retry_after
            );
        }
        crate::gemini::GeminiError::QuotaExceeded { limit } => {
            println!(
                "{} Quota exceeded: {}. Please check your Gemini API usage.",
                "ERROR:".red(),
                limit
            );
        }
        crate::gemini::GeminiError::ModelNotFound { model } => {
            println!(
                "{} Model '{}' not found. Please check the model name in the configuration.",
                "ERROR:".red(),
                model
            );
        }
        crate::gemini::GeminiError::InvalidApiKey => {
            println!(
                "{} Invalid API key. Please check your GEMINI_API_KEY environment variable.",
                "ERROR:".red()
            );
        }
        crate::gemini::GeminiError::ContentPolicyViolation { reason } => {
            println!("{} Content policy violation: {}", "ERROR:".red(), reason);
        }
        crate::gemini::GeminiError::ServiceUnavailable { reason } => {
            println!(
                "{} Gemini service is temporarily unavailable: {}",
                "ERROR:".red(),
                reason
            );
        }
        crate::gemini::GeminiError::NetworkError(e) => {
            println!("{} Network error: {}", "ERROR:".red(), e);
        }
        crate::gemini::GeminiError::Timeout { seconds } => {
            println!(
                "{} Request timed out after {} seconds.",
                "ERROR:".red(),
                seconds
            );
        }
        crate::gemini::GeminiError::InvalidRequest { details } => {
            println!("{} Invalid request: {}", "ERROR:".red(), details);
        }
        crate::gemini::GeminiError::ApiError { status, message } => {
            println!(
                "{} API error (HTTP {}): {}",
                "ERROR:".red(),
                status,
                message
            );
        }
        crate::gemini::GeminiError::InvalidResponse(msg) => {
            println!("{} Invalid response from Gemini: {}", "ERROR:".red(), msg);
        }
        crate::gemini::GeminiError::InternalError { details } => {
            println!("{} Internal server error: {}", "ERROR:".red(), details);
        }
        crate::gemini::GeminiError::SerializationError(e) => {
            println!("{} JSON serialization error: {}", "ERROR:".red(), e);
        }
    }

    println!("\n{} Check the following:", "HINT:".yellow());
    println!("  - Your GEMINI_API_KEY is correctly set");
    println!("  - Your internet connection is working");
    println!("  - Gemini API service is available");
    println!("  - You haven't exceeded your API quota");
}
