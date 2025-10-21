use crate::bindings::pdfjs;
use leptos::{prelude::*, task::spawn_local};
use uuid::Uuid;

#[component]
pub fn PdfRenderer(
    #[prop(into)] url: MaybeProp<String>,
    #[prop(default = 1usize.into(), into)] page: MaybeProp<usize>,
    #[prop(default = 1f32.into(), into)] scale: MaybeProp<f32>,
    #[prop(default = true.into(), into)] text: MaybeProp<bool>,
) -> impl IntoView {
    // Generate unique IDs for PDF.js layers
    let id = Uuid::new_v4().to_string();
    let shared_canvas_id = format!("pdf-canvas-{id}");
    let shared_text_layer_id = format!("pdf-canvas-text-layer-{id}");
    // Clone for Leptos elements
    let canvas_id = shared_canvas_id.clone();
    let text_layer_id = shared_text_layer_id.clone();
    let pdf_handle = LocalResource::new(move || {
        let canvas_id_local_resource = shared_canvas_id.clone();
        let text_layer_id = shared_text_layer_id.clone();
        let url_local_resource = url.get().unwrap_or(String::new());
        async move {
            pdfjs::pdf::load_pdf(
                &canvas_id_local_resource,
                &url_local_resource,
                &text_layer_id,
            )
            .await
            .inspect_err(|e| log::error!("Unable to load PDF: {:?}", e))
            .ok()
        }
    });
    let _on_change = Effect::new(move || {
        let current_page = page.get().take_if(|v| *v > 0).unwrap_or_else(|| {
            log::warn!("PdfRenderer page must be a positive integer. Setting to first page");
            1usize
        });
        let current_scale = scale.get().take_if(|v| *v > 0f32).unwrap_or_else(|| {
            log::warn!("PdfRenderer scale must be a positive float. Setting to default scale");
            1f32
        });
        let render_text_layer = text.get().unwrap_or(true);
        if let Some(resource) = pdf_handle.get() {
            if let Some(handle) = resource {
                spawn_local(async move {
                    if let Err(e) = handle
                        .render_page(current_page, current_scale, render_text_layer)
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
        <div class="pdf-layers" id=format!("pdf-renderer-{id}")>
            <div class="pdf-layer__canvas">
                <canvas id=canvas_id style:position="absolute" />
            </div>
            <div class="pdf-layer__text" id=text_layer_id />
        </div>
    }
}
