use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid token. Deleted credentials file")]
pub struct InvalidTokenError {
    pub source: anyhow::Error,
}

#[derive(Debug, Error)]
#[error("Error running tests on sandbox")]
pub struct SandboxTestError {
    pub path: Option<PathBuf>,
    pub source: anyhow::Error,
}

#[derive(Debug, Error)]
#[error("Failed to download one or more exercises")]
pub struct DownloadsFailedError {
    pub completed: Vec<usize>,
    pub skipped: Vec<usize>,
    pub failed: Vec<(usize, Vec<String>)>,
}
