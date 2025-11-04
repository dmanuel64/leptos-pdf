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

use std::{thread::sleep, time::Duration};

use crate::components::PdfPage;
use leptos::{ev, html::Div, prelude::*, task::spawn_local};
use leptos_meta::*;
use pdfium_render::prelude::*;
use uuid::Uuid;
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use wasm_bindgen_futures::js_sys;

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
    let (loaded, set_loaded) = signal(false);
    Effect::new(move |_| {
        let window = window();

        // Create a Closure that handles the event
        let closure = Closure::wrap(Box::new(move |_event: ev::Event| {
            wasm_bindgen_futures::spawn_local(async move {
                // Wait 500ms for the WASM module to finish initializing
                log::info!("Trunk application started (PDFium WASM loaded)");
                set_loaded.set(true);
            });
        }) as Box<dyn FnMut(_)>);

        // Add the event listener
        window
            .add_event_listener_with_callback(
                "PdfiumRenderInitialized",
                closure.as_ref().unchecked_ref(),
            )
            .expect("failed to add event listener");

        // Leak the closure to keep it alive for the lifetime of the app
        closure.forget();
    });

    // Regular Leptos view
    view! {
        <Show when=move || loaded.get() fallback=move || view! { <div>"Loading PDF renderer..."</div> }>
            {let pdfium = Pdfium::default();
            log::info!("Pdfium initialized: {:?}", pdfium);}
            </Show>
    }
}
