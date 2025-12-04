use std::rc::Rc;

use crate::{
    components::{
        pdf_document::{DocumentViewerLayout, TextLayerConfig},
        PdfDocument,
    },
    errors::PdfError,
};
use leptos::{ev::error, prelude::*};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::Uint8Array, Response};

async fn fetch_bytes(url: &str) -> Result<Vec<u8>, JsValue> {
    let window = window();

    // Fetch the PDF
    let resp_value = JsFuture::from(window.fetch_with_str(url)).await?;
    let resp: Response = resp_value
        .dyn_into()
        .expect("The return value type of fetch should be a Response");

    // Await the array buffer from the response
    let abuf_promise = resp.array_buffer()?;
    let abuf = JsFuture::from(abuf_promise).await?;

    // Convert ArrayBuffer → Uint8Array → Vec<u8>
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
    /// Configuration options for the selectable text layer. Not providing a value will not render a text layer
    #[prop(optional, into)]
    text_layer_config: MaybeProp<TextLayerConfig>,
    /// Optional signal to capture all text in the document
    // #[prop(optional, into)]
    // set_captured_document_text: Option<WriteSignal<Vec<String>>>,
    /// Layout options for the document viewer
    #[prop(optional, into)]
    viewer_layout: Signal<DocumentViewerLayout>,
    /// Whether to show a scrollbar when the content overflows
    #[prop(default=true.into(), into)]
    scrollbar: Signal<bool>,
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
                        Result<AnyView, PdfError>,
                    >::new(async move {
                        let pdf_bytes = pdf_bytes
                            .await
                            .map_err(|e| PdfError::LoadingError(format!("{:?}", e)))?;
                        Ok(
                            view! {
                                <PdfDocument
                                    pdf_bytes=pdf_bytes.clone()
                                    password=password
                                    error_fallback=error_fallback.clone()
                                    text_layer_config=text_layer_config
                                    // set_captured_document_text=set_captured_document_text
                                    viewer_layout=viewer_layout
                                    scrollbar=scrollbar
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
