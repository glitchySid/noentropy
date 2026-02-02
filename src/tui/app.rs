use crate::files::FileBatch;
use crate::models::{FileCategory, OrganizationPlan};
use crate::settings::Config;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    /// Initial state - scanning files
    Scanning,
    /// Displaying file list, waiting for user to start organization
    FileList,
    /// Fetching AI categorization
    Fetching,
    /// Showing the organization plan for confirmation
    PlanReview,
    /// Moving files
    Moving,
    /// Organization complete
    Done,
    /// Error state
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Files,
    Plan,
    Progress,
}

pub struct App {
    pub state: AppState,
    pub tab: Tab,
    pub config: Config,
    pub target_path: PathBuf,
    pub recursive: bool,
    pub dry_run: bool,
    pub offline: bool,

    // Online mode state
    pub online_requested: bool,
    pub online_available: bool,

    // File data
    pub batch: Option<FileBatch>,
    pub plan: Option<OrganizationPlan>,

    // UI state
    pub file_list_state: usize,
    pub plan_list_state: usize,
    pub scroll_offset: usize,

    // Progress
    pub progress: usize,
    pub total_files: usize,
    pub moved_count: usize,
    pub error_count: usize,

    // Status message
    pub status_message: String,

    // Should quit
    pub should_quit: bool,
}

impl App {
    pub fn new(config: Config, target_path: PathBuf, recursive: bool, dry_run: bool) -> Self {
        // Initialize offline-first based on config preference
        let online_requested = config.prefer_online;

        Self {
            state: AppState::Scanning,
            tab: Tab::Files,
            config,
            target_path,
            recursive,
            dry_run,
            offline: !online_requested,
            online_requested,
            online_available: false,
            batch: None,
            plan: None,
            file_list_state: 0,
            plan_list_state: 0,
            scroll_offset: 0,
            progress: 0,
            total_files: 0,
            moved_count: 0,
            error_count: 0,
            status_message: String::from("Scanning files..."),
            should_quit: false,
        }
    }

    pub fn scan_files(&mut self) {
        let batch = FileBatch::from_path(&self.target_path, self.recursive);
        self.total_files = batch.count();

        if self.total_files == 0 {
            self.state = AppState::Error("No files found to organize".to_string());
            self.status_message = "No files found".to_string();
        } else {
            self.batch = Some(batch);
            self.state = AppState::FileList;
            self.status_message = format!("Found {} files", self.total_files);
        }
    }

    pub fn set_plan(&mut self, plan: OrganizationPlan) {
        self.plan = Some(plan);
        self.state = AppState::PlanReview;
        self.tab = Tab::Plan;
        self.status_message = "Review the organization plan".to_string();
    }

    pub fn set_error(&mut self, error: String) {
        self.state = AppState::Error(error.clone());
        self.status_message = error;
    }

    pub fn start_fetching(&mut self) {
        self.state = AppState::Fetching;
        self.status_message = "Fetching AI categorization...".to_string();
    }

    pub fn start_moving(&mut self) {
        self.state = AppState::Moving;
        self.tab = Tab::Progress;
        self.progress = 0;
        self.moved_count = 0;
        self.error_count = 0;
        self.status_message = "Moving files...".to_string();
    }

    pub fn update_progress(&mut self, moved: usize, errors: usize) {
        self.moved_count = moved;
        self.error_count = errors;
        self.progress = moved + errors;
        self.status_message = format!("Moved {}/{} files", self.progress, self.total_files);
    }

    pub fn finish(&mut self) {
        self.state = AppState::Done;
        self.status_message = format!(
            "Done! Moved: {}, Errors: {}",
            self.moved_count, self.error_count
        );
    }

    pub fn next_file(&mut self) {
        if let Some(ref batch) = self.batch
            && self.file_list_state < batch.filenames.len().saturating_sub(1)
        {
            self.file_list_state += 1;
        }
    }

    pub fn previous_file(&mut self) {
        if self.file_list_state > 0 {
            self.file_list_state -= 1;
        }
    }

    pub fn next_plan_item(&mut self) {
        if let Some(ref plan) = self.plan
            && self.plan_list_state < plan.files.len().saturating_sub(1)
        {
            self.plan_list_state += 1;
        }
    }

    pub fn previous_plan_item(&mut self) {
        if self.plan_list_state > 0 {
            self.plan_list_state -= 1;
        }
    }

    pub fn next_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Files => Tab::Plan,
            Tab::Plan => Tab::Progress,
            Tab::Progress => Tab::Files,
        };
    }

    pub fn previous_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Files => Tab::Progress,
            Tab::Plan => Tab::Files,
            Tab::Progress => Tab::Plan,
        };
    }

    pub fn get_selected_file(&self) -> Option<(&String, &PathBuf)> {
        self.batch.as_ref().and_then(|batch| {
            batch
                .filenames
                .get(self.file_list_state)
                .zip(batch.paths.get(self.file_list_state))
        })
    }

    pub fn get_selected_plan_item(&self) -> Option<&FileCategory> {
        self.plan
            .as_ref()
            .and_then(|plan| plan.files.get(self.plan_list_state))
    }
}
