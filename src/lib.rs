//! # `leptos-pdf`
//!
//! `leptos-pdf` provides utilities for rendering PDFs as components in Leptos.
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
mod bindings;
pub mod components;

pub use components::PdfRenderer;
