//! Provides the [`PdfRenderer`] component that renders PDF documents in the browser using the
//! underlying PDF.js bindings.
//! ## Example
//!
//! ```rust
//! use leptos::*;
//! use leptos_pdf::PdfRenderer;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <PdfRenderer
//!             url="https://example.com/sample.pdf"
//!             page=1
//!             scale=1.25
//!             text=true
//!         />
//!     }
//! }
//! ```

use leptos::prelude::*;
use pdfium_render::prelude::*;

/// A rendered PDF document.
#[component]
pub fn PdfRenderer(
    /// URL to the PDF file.
    #[prop(into)]
    url: MaybeProp<String>,
    /// Which page to render.
    #[prop(default = 1usize.into(), into)]
    page: MaybeProp<usize>,
    /// Scale of the PDF page.
    #[prop(default = 1f32.into(), into)]
    scale: MaybeProp<f32>,
    /// `true` if text content should be extracted from the PDF and overlayed on the canvas for
    /// text selection.
    #[prop(default = true.into(), into)]
    text: MaybeProp<bool>,
    // #[prop(default = true.into(), into)] annotations: MaybeProp<bool>,
) -> impl IntoView {
    // Regular Leptos view
    view! {
        // <h1>"My Data"</h1>
        // <Suspense fallback=move || view! { <p>"Loading..."</p> }>
        //     <h2>"My Data"</h2>
        //     {move || { pdfium.with(|p| {
        //         if let Some(pdf) = p.as_ref() {
        //             let result = pdf.load_pdf_from_byte_slice(&[1], None);
        //             log::warn!("Loaded PDF: {:?}", result);
        //             true
        //         } else {
        //             false
        //         }
                
        //     }).then(|| view! { <p>"PDFium initialized"</p> }) }}
        // </Suspense>
    }
}
