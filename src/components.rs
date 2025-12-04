//! Components for rendering PDFs.

mod pdf_document;
mod pdf_page;
mod pdf_viewer;
mod pdfium;

// TODO: figure out if props should be in a different module
pub use pdf_document::PdfDocument;
pub use pdf_document::TextLayerConfig;
use pdf_page::PdfPage;
pub use pdf_viewer::PdfViewer;
pub use pdfium::PdfiumProvider;
