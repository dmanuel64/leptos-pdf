use std::rc::Rc;

use leptos::{html::Canvas, prelude::*};
use pdfium_render::prelude::Pdfium;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{js_sys, JsFuture};
use web_sys::{js_sys::Uint8Array, RequestMode, Response};

use crate::components::pdfium::PdfiumInjection;

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

#[component]
pub fn PdfDocument(
    #[prop(into)] url: Signal<String>,
    #[prop(optional, into)] password: MaybeProp<String>,
    #[prop(optional, into)] loading_fallback: ViewFnOnce,
    #[prop(optional, into)] error_fallback: ViewFn,
    #[prop(default=RequestMode::SameOrigin)] mode: RequestMode,
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
                    let pdfium_ref = pdfium.borrow();
                    if let Some(pdf_data_result) = pdf_data.get() {
                        if let Some(pdf_data) = pdf_data_result.take() {
                            let view = match pdfium_ref
                                .load_pdf_from_byte_vec(pdf_data, password.get().as_deref())
                            {
                                Ok(pdf) => {
                                    for page in pdf.pages().iter() {
                                        page.render(2000, 2000, None).unwrap().as_image().as_bytes();
                                        // canvas_ref.get().unwrap().get_context("2d");
                                        log::warn!("Rendered");
                                    }
                                    Some(
                                        view! {
                                            <canvas node_ref=canvas_ref>
                                                <p>{"The PDF"}</p>
                                            </canvas>
                                        }
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
