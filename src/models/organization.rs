use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileCategory {
    pub filename: String,
    pub category: String,
    pub sub_category: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationPlan {
    pub files: Vec<FileCategory>,
}
