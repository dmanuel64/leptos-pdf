use thiserror::Error;

#[derive(Debug, Error)]
pub enum PdfError {
    #[error("Failed to load PDF: {0}")]
    LoadingError(String),
    #[error("Failed to render PDF: {0}")]
    RenderError(String),
    #[error("Failed to extract text from PDF: {0}")]
    TextExtractionError(String),
}

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("Failed to fetch blob: {0}")]
    FailedToFetch(String),
}
