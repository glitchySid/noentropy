use crate::gemini::types::OrganizationPlanResponse;
use crate::models::{FileCategory, OrganizationPlan};

impl OrganizationPlanResponse {
    pub fn to_organization_plan(self) -> OrganizationPlan {
        OrganizationPlan {
            files: self
                .files
                .into_iter()
                .map(|f| FileCategory {
                    filename: f.filename,
                    category: f.category,
                    sub_category: String::new(),
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct PromptBuilder {
    file_list: String,
}

impl PromptBuilder {
    pub fn new(file_list: &[String]) -> Self {
        Self {
            file_list: file_list.join(", "),
        }
    }

    pub fn build_categorization_prompt(&self, categories: &[String]) -> String {
        let categories_str = categories.join("', '");
        format!(
            "I have these files in my Downloads folder: [{}]. \
             Categorize them into these folders: '{}'. \
             Return ONLY a JSON object with this structure: {{ 'files': [ {{ 'filename': 'name', 'category': 'folder' }} ] }}",
            self.file_list, categories_str
        )
    }

    pub fn build_subcategory_prompt(
        filename: &str,
        parent_category: &str,
        content: &str,
    ) -> String {
        format!(
            "I have a file named '{}' inside the '{}' folder. Here is the first 1000 characters of content:\n---\n{}\n---\nBased on this, suggest a single short sub-folder name (e.g., 'Invoices', 'Notes', 'Config'). Return ONLY the name of the sub-folder. Do not use markdown or explanations.",
            filename, parent_category, content
        )
    }
}
