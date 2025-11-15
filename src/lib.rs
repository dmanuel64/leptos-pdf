//! # `leptos-pdf`
//!
//! `leptos-pdf` is a lightweight Leptos component library for rendering and viewing PDF files
//! directly in your browser using [PDF.js](https://mozilla.github.io/pdf.js/).
//!
//! It provides an idiomatic Leptos interface for embedding PDFs in your Rust + WebAssembly
//! applications - complete with canvas-based renderings, text selection, and reactive props for paging and scaling.
//!
//! ## Example
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_pdf::PdfRenderer;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <div style:width="100vw" style:height="100vh">
//!             <PdfRenderer url="/public/sample.pdf"/>
//!         </div>
//!     }
//! }
//!
//! fn main() {
//!     mount_to_body(App)
//! }
//! ```
//!
//! See the [`PdfRenderer`] component for more information on PDF rendering.

pub mod components;
pub use components::PdfRenderer;
pub use components::PdfDocument;
pub use components::PdfiumProvider;