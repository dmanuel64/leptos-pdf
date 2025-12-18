//! Contains the [`PdfViewer`] component for rendering PDFs in a Leptos app.

use crate::{components::PdfDocument, errors::FetchError};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Response, js_sys::Uint8Array};

/// Fetches bytes of a remote blob (e.g., PDF file) from the given URL
async fn fetch_bytes(url: &str) -> Result<Vec<u8>, JsValue> {
    let window = window();

    let resp_value = JsFuture::from(window.fetch_with_str(url)).await?;
    let resp: Response = resp_value
        .dyn_into()
        .expect("The return value type of fetch should be a Response");

    // Await the array buffer from the response
    let abuf_promise = resp.array_buffer()?;
    let abuf = JsFuture::from(abuf_promise).await?;

    // Convert ArrayBuffer to JavaScript Uint8Array then to a Rust Vec<u8>
    let u8_array = Uint8Array::new(&abuf);
    let bytes = u8_array.to_vec();
    Ok(bytes)
}

/// A Component that fetches and renders a PDF document from a URL.
#[component]
pub fn PdfViewer<FalFn, Fal>(
    /// URL to the PDF file
    #[prop(into)]
    url: Signal<String>,
    /// View to display while the PDF is loading
    #[prop(optional, into)]
    loading_fallback: ViewFnOnce,
    /// Password to open the PDF, if required.
    #[prop(optional, into)]
    password: MaybeProp<String>,
    /// View to display if an error is encountered and the PDF cannot be loaded.
    error_fallback: FalFn,
    /// Padding around the pages.
    #[prop(default="20px".into(), into)]
    padding: Signal<String>,
    /// Gap between pages.
    #[prop(default="20px".into(), into)]
    gap: Signal<String>,
    /// Background color behind the pages.
    #[prop(default="gray".into(), into)]
    background: Signal<String>,
    /// Zoom factor applied when rendering each page.
    #[prop(default=0.5.into(), into)]
    page_zoom: Signal<f32>,
    /// Whether the document container should display a scrollbar.
    #[prop(default=true.into(), into)]
    show_scrollbar: Signal<bool>,
    /// Optional sink for extracted text: each page's text is appended in order.
    #[prop(optional, into)]
    document_text: RwSignal<Option<Vec<String>>>,
    // #[prop(optional, into)] enable_text_layer: Signal<bool>,
    // #[prop(optional, into)] use_precise_font_size: Signal<bool>,
    // #[prop(optional, into)] use_precise_char_bounds: Signal<bool>,
    // #[prop(optional, into)] use_precise_line_bounds: Signal<bool>,
    // #[prop(optional, into)] font_size_match: Signal<FontSizeMatch>,
    // #[prop(default=false.into(), into)] require_same_font: Signal<bool>,
) -> impl IntoView
where
    FalFn: FnMut(ArcRwSignal<Errors>) -> Fal + Send + Clone + 'static,
    Fal: IntoView + Send + 'static,
{
    let pdf_bytes = LocalResource::new(move || async move { fetch_bytes(&url.get()).await });
    view! {
        <Transition fallback=loading_fallback>
            <ErrorBoundary fallback=error_fallback
                .clone()>
                {move || {
                    let error_fallback = error_fallback.clone();
                    Suspend::<
                        Result<AnyView, FetchError>,
                    >::new(async move {
                        let pdf_bytes = pdf_bytes
                            .await
                            .map_err(|e| FetchError::FailedToFetch(format!("{:?}", e)))?;
                        Ok(
                            view! {
                                <PdfDocument
                                    pdf_bytes
                                    password
                                    error_fallback=error_fallback.clone()
                                    padding
                                    gap
                                    background
                                    page_zoom
                                    show_scrollbar
                                    document_text
                                />
                            }
                                .into_any(),
                        )
                    })
                }}
            </ErrorBoundary>
        </Transition>
    }
}
