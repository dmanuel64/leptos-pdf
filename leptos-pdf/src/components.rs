//! UI components for rendering PDF documents in a Leptos application.
//!
//! This module defines the public component surface for PDF rendering, ranging from
//! low-level building blocks to higher-level convenience components.
//!
//! ## Component overview
//!
//! - [`PdfViewer`]: High-level component that fetches a PDF from a URL and renders it.
//! - [`PdfDocument`]: Low-level component for rendering a PDF from raw bytes.
//! - [`PdfiumProvider`]: Provides access to the PDFium runtime used for rendering.
//! - `PdfPage`: Internal component for rendering a single PDF page to a `<canvas>`.
//!
//! Most applications should start with [`PdfViewer`]. Use [`PdfDocument`] directly when
//! you already have the PDF bytes or need a custom loading pipeline.

mod pdf_document;
mod pdf_page;
mod pdf_viewer;
mod pdfium;

pub use pdf_document::PdfDocument;
use pdf_page::PdfPage;
pub use pdf_viewer::PdfViewer;
pub use pdfium::PdfiumProvider;
