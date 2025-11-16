//! Components for rendering PDFs.

mod pdf_document;
mod pdf_page;
mod pdf_renderer;
mod pdf_viewer;
mod pdfium;

pub use pdf_document::PdfDocument;
pub use pdf_document::TextLayerConfig;
use pdf_page::PdfPage;
pub use pdf_renderer::PdfRenderer;
pub use pdfium::PdfiumProvider;
