use crate::bindings::pdfjs;
use leptos::{prelude::*, task::spawn_local};
use uuid::Uuid;

#[component]
pub fn PdfViewer(
    url: &'static str,
    #[prop(default = 1usize.into(), into)] page: MaybeProp<usize>,
    #[prop(optional, into)] width: MaybeProp<usize>,
    #[prop(optional, into)] height: MaybeProp<usize>,
) -> impl IntoView {
    let id = format!("pdf-canvas-{}", Uuid::new_v4());
    let canvas_id = id.clone();
    let pdf_handle = LocalResource::new(move || {
        let canvas_id_local_resource = id.clone();
        async move {
            pdfjs::pdf::load_pdf(&canvas_id_local_resource, url)
                .await
                .inspect_err(|e| log::error!("Unable to load PDF: {:?}", e))
                .ok()
        }
    });
    let _on_page_change = Effect::new(move || {
        let current_page = page.get();
        if let Some(resource) = pdf_handle.get() {
            if let Some(handle) = resource {
                spawn_local(async move {
                    if let Err(e) = handle.render_page(current_page.unwrap_or(1)).await {
                        log::error!("Error on changing page: {:?}", e);
                    }
                });
            } else {
                log::error!("Cannot change pages - PDFHandle did not load correctly")
            }
        }
    });
    
    view! {
        <canvas id=canvas_id width height/>
    }
}
