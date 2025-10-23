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

use crate::bindings::pdfjs;
use leptos::{prelude::*, task::spawn_local};
use uuid::Uuid;

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
    // Generate unique IDs for PDF.js layers
    let id = Uuid::new_v4().to_string();
    let shared_canvas_id = format!("pdf-canvas-{id}");
    let shared_text_layer_id = format!("pdf-canvas-text-layer-{id}");
    let shared_annotation_layer_id = format!("pdf-canvas-annotation-layer-{id}");
    // Clone for Leptos elements
    let canvas_id = shared_canvas_id.clone();
    let text_layer_id = shared_text_layer_id.clone();
    let annotation_layer_id = shared_annotation_layer_id.clone();
    let pdf_handle = LocalResource::new(move || {
        // Clone for PDF.js elements
        let local_resource_canvas_id = shared_canvas_id.clone();
        let local_resource_text_layer_id = shared_text_layer_id.clone();
        let local_resource_url = url.get().unwrap_or(String::new());
        let local_resource_annotation_layer_id = shared_annotation_layer_id.clone();
        async move {
            pdfjs::pdf::load_pdf(
                &local_resource_canvas_id,
                &local_resource_url,
                &local_resource_text_layer_id,
                &local_resource_annotation_layer_id,
            )
            .await
            .inspect_err(|e| log::error!("Unable to load PDF: {:?}", e))
            .ok()
        }
    });
    let _on_change = Effect::new(move || {
        // Validate positive integers
        let current_page = page.get().take_if(|v| *v > 0).unwrap_or_else(|| {
            log::warn!("PdfRenderer page must be a positive integer. Setting to first page");
            1usize
        });
        let current_scale = scale.get().take_if(|v| *v > 0f32).unwrap_or_else(|| {
            log::warn!("PdfRenderer scale must be a positive float. Setting to default scale");
            1f32
        });
        let render_text_layer = text.get().unwrap_or(true);
        let render_annotation_layer = false;
        if let Some(resource) = pdf_handle.get() {
            if let Some(handle) = resource {
                spawn_local(async move {
                    if let Err(e) = handle
                        .render_page(
                            current_page,
                            current_scale,
                            render_text_layer,
                            render_annotation_layer,
                        )
                        .await
                    {
                        log::error!("Error on changing page: {:?}", e);
                    }
                });
            } else {
                log::error!("Cannot change pages - PDFHandle did not load correctly")
            }
        }
    });

    view! {
        // Include style for correct overlaying of text and annotations
        <style>{include_str!("../../styles/pdf_renderer.scss")}</style>
        <div class="pdf-layers" id=format!("pdf-renderer-{id}")>
            <div class="pdf-layer__canvas">
                <canvas id=canvas_id />
            </div>
            <div class="pdf-layer__text" id=text_layer_id />
        // <div class="pdf-layer__annotation" id=annotation_layer_id />
        </div>
    }
}