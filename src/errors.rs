use thiserror::Error;

#[derive(Debug, Error)]
pub enum PdfError {
    #[error("Failed to load PDF: {0}")]
    LoadingError(String),
}
