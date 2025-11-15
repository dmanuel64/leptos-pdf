use std::rc::Rc;

use leptos::{html::Canvas, prelude::*};
use pdfium_render::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{js_sys, JsFuture};
use web_sys::{js_sys::Uint8Array, RequestMode, Response};

use crate::components::{
    pdf_page::{PdfPage, TextWord},
    pdfium::PdfiumInjection,
};

async fn fetch_pdf_bytes(url: &str) -> Result<Vec<u8>, JsValue> {
    let window = web_sys::window().unwrap();

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
pub fn PdfDocument(
    #[prop(into)] url: Signal<String>,
    #[prop(optional, into)] password: MaybeProp<String>,
    #[prop(optional, into)] loading_fallback: ViewFnOnce,
    #[prop(optional, into)] error_fallback: ViewFn,
    #[prop(default=RequestMode::SameOrigin)] mode: RequestMode,
    #[prop(optional)] text_layer_config: Option<TextLayerConfig>,
) -> impl IntoView {
    let pdfium = LocalResource::new(move || {
        let injection = PdfiumInjection::use_context()
            .expect("PdfDocument must be used within a PdfiumProvider component");
        async move { injection.create_pdfium().await }
    });
    let pdf_data = LocalResource::new(move || async move {
        fetch_pdf_bytes(&url.get())
            .await
            .inspect_err(|err| log::error!("Failed to fetch PDF {}: {:?}", url.get(), err))
            .ok()
    });
    let canvas_ref = NodeRef::<Canvas>::new();

    view! {
        <Transition fallback=loading_fallback>
            {move || {
                if let Some(pdfium) = pdfium.get() {
                    if let Some(pdf_data_result) = pdf_data.get() {
                        if let Some(pdf_data) = pdf_data_result.take() {
                            let view = match pdfium
                                .load_pdf_from_byte_vec(pdf_data, password.get().as_deref())
                            {
                                Ok(pdf) => {
                                    Some(
                                        pdf
                                            .pages()
                                            .iter()
                                            .enumerate()
                                            .map(|(idx, page)| {
                                                let page_num = idx + 1;
                                                if let Some(_text_config) = text_layer_config {
                                                    let words = page
                                                        .text()
                                                        .map(|text| {
                                                            let segments = text.segments();
                                                            let words: Vec<TextWord> = segments
                                                                .iter()
                                                                .map(|segment| TextWord::from(segment))
                                                                .collect();
                                                            words
                                                        })
                                                        .inspect_err(|e| {
                                                            log::error!(
                                                                "Failed to extract PDF text from page {page_num}: {}", e
                                                            )
                                                        })
                                                        .unwrap_or_default();
                                                    view! { <PdfPage words /> }
                                                } else {
                                                    view! { <PdfPage /> }
                                                }
                                            })
                                            .collect_view()
                                            .into_any(),
                                    )
                                }
                                Err(err) => {
                                    log::error!("Failed to load PDF {}: {:?}", url.get(), err);
                                    Some(error_fallback.run().into_any())
                                }
                            };
                            view
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }}
        </Transition>
    }
}
