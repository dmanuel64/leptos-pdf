use crate::{components::PdfDocument, errors::FetchError};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::Uint8Array, Response};

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

#[component]
pub fn PdfViewer<FalFn, Fal>(
    /// URL to the PDF file
    #[prop(into)]
    url: Signal<String>,
    /// Password to access the PDF if required
    #[prop(optional, into)]
    password: MaybeProp<String>,
    /// View to display while the PDF is loading
    #[prop(optional, into)]
    loading_fallback: ViewFnOnce,
    /// View to display if an error is encountered and the PDF cannot be loaded
    error_fallback: FalFn,
    #[prop(default="20px".into(), into)] padding: Signal<String>,
    #[prop(default="20px".into(), into)] gap: Signal<String>,
    #[prop(default="gray".into(), into)] background: Signal<String>,
    #[prop(default=0.5.into(), into)] page_scale: Signal<f32>,
    #[prop(default=true.into(), into)] scrollbar: Signal<bool>,
    #[prop(optional, into)] document_text: RwSignal<Option<Vec<String>>>,
    #[prop(optional, into)] enable_text_layer: Signal<bool>,
    #[prop(optional, into)] use_precise_font_size: Signal<bool>,
    #[prop(optional, into)] use_precise_char_bounds: Signal<bool>,
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
                                    pdf_bytes=pdf_bytes.clone()
                                    password=password
                                    error_fallback=error_fallback.clone()
                                    padding=padding
                                    gap=gap
                                    background=background
                                    page_zoom=page_scale
                                    show_scrollbar=scrollbar
                                    pdf_text=document_text
                                    enable_text_layer=enable_text_layer
                                    use_precise_font_size=use_precise_font_size
                                    use_precise_char_bounds=use_precise_char_bounds
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
