//! Error types used by [`leptos-pdf`](crate).

use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys;

/// Errors that can occur while initializing PDFium, or loading, rendering, or processing a
/// PDF document.
#[derive(Debug, Error)]
pub enum PdfError {
    /// Failed to load or open the PDF document.
    ///
    /// This typically occurs when the PDF bytes are invalid, corrupted, or when an
    /// incorrect password is provided for an encrypted document.
    #[error("Failed to load PDF: {0}")]
    LoadingError(String),
    /// Failed while rendering a PDF page into a raster image.
    ///
    /// This may be caused by invalid page dimensions, rendering configuration issues,
    /// or internal PDFium errors.
    #[error("Failed to render PDF: {0}")]
    RenderError(String),
    /// Failed to extract text from a PDF page.
    ///
    /// This can occur if PDFium encounters an internal error during text processing.
    #[error("Failed to extract text from PDF: {0}")]
    TextExtractionError(String),
    #[error("Failed to initialize PDFium: {0}")]
    PdfiumInitError(String),
}

pub fn js_error_string_or(value: JsValue, custom: String) -> String {
    value
        .dyn_into::<js_sys::Error>()
        .ok()
        .and_then(|e| e.to_string().as_string())
        .unwrap_or(custom)
}

/// Errors that can occur while fetching a remote PDF.
#[derive(Debug, Error)]
pub enum FetchError {
    /// Failed to fetch a remote binary blob (such as a PDF file).
    ///
    /// This may be caused by network errors, CORS issues, invalid URLs,
    /// or failures in the browser `fetch` API.
    #[error("Failed to fetch blob: {0}")]
    FailedToFetch(String),
}
