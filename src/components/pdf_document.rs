use leptos::prelude::*;
use pdfium_render::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::Uint8Array, RequestMode, Response};

use crate::{
    components::{pdf_page::PageWord, pdfium::PdfiumInjection, PdfPage},
    errors::PdfError,
};

async fn fetch_pdf_bytes(url: &str) -> Result<Vec<u8>, JsValue> {
    let window = window();

    // Fetch the PDF
    let resp_value = JsFuture::from(window.fetch_with_str(url)).await?;
    let resp: Response = resp_value.dyn_into()?;

    // Await the array buffer from the response
    let abuf_promise = resp.array_buffer()?;
    let abuf = JsFuture::from(abuf_promise).await?;

    // Convert ArrayBuffer → Uint8Array → Vec<u8>
    let u8_array = Uint8Array::new(&abuf);
    let bytes = u8_array.to_vec();
    Ok(bytes)
}

#[derive(Debug, Clone, Copy)]
pub struct TextLayerConfig {}

#[component]
pub fn PdfDocument<FalFn, Fal>(
    #[prop(into)] url: Signal<String>,
    #[prop(optional, into)] password: MaybeProp<String>,
    #[prop(optional, into)] loading_fallback: ViewFnOnce,
    error_fallback: FalFn,
    #[prop(default=RequestMode::SameOrigin)] mode: RequestMode,
    #[prop(optional, into)] text_layer_config: MaybeProp<TextLayerConfig>,
    #[prop(into, default=1f32.into())] scale: Signal<f32>,
) -> impl IntoView
where
    FalFn: FnMut(ArcRwSignal<Errors>) -> Fal + Send + 'static,
    Fal: IntoView + Send + 'static,
{
    let pdfium = LocalResource::new(move || {
        let injection = PdfiumInjection::use_context()
            .expect("PdfDocument must be used within a PdfiumProvider component");
        async move { injection.create_pdfium().await }
    });
    let pdf_data = LocalResource::new(move || async move { fetch_pdf_bytes(&url.get()).await });
    view! {
        <div class="leptos-pdf-document">
            <ErrorBoundary fallback=error_fallback>
                <Transition fallback=loading_fallback>
                    {move || Suspend::<
                        Result<Vec<AnyView>, PdfError>,
                    >::new(async move {
                        let pdfium = pdfium.await;
                        let pdf_data = pdf_data.await;
                        let pdf = pdfium
                            .load_pdf_from_byte_vec(pdf_data.unwrap(), password.get().as_deref())
                            .map_err(|e| PdfError::LoadingError(format!("{}", e)))?;
                        let mut views: Vec<AnyView> = Vec::new();
                        for page in pdf.pages().iter() {
                            let words: Vec<PageWord> = if let Some(text_layer_config) = text_layer_config
                                .get()
                            {
                                vec![]
                            } else {
                                vec![]
                            };
                            let rendered_page = page
                                .render_with_config(
                                    &PdfRenderConfig::new().scale_page_by_factor(scale.get()),
                                )
                                .map_err(|e| PdfError::RenderError(format!("{}", e)))?
                                .as_image_data()
                                .map_err(|e| PdfError::RenderError(format!("{:?}", e)))?;
                            views.push(view! { <PdfPage rendered_page words /> }.into_any());
                        }
                        Ok(views)
                    })}
                </Transition>
            </ErrorBoundary>
        </div>
    }
}
