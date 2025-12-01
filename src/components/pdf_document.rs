use leptos::{html::Div, prelude::*};
use leptos_use::{use_element_size, UseElementSizeReturn};
use pdfium_render::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::Uint8Array, RequestInit, RequestMode, Response};

use crate::{
    components::{pdf_page::PdfText, pdfium::PdfiumInjection, PdfPage},
    errors::PdfError,
};

async fn fetch_pdf_bytes(url: &str, mode: RequestMode) -> Result<Vec<u8>, JsValue> {
    let window = window();

    // Fetch the PDF
    let request_init = RequestInit::new();
    request_init.set_mode(mode);
    let resp_value = JsFuture::from(window.fetch_with_str_and_init(url, &request_init)).await?;
    let resp: Response = resp_value.dyn_into()?;

    // Await the array buffer from the response
    let abuf_promise = resp.array_buffer()?;
    let abuf = JsFuture::from(abuf_promise).await?;

    // Convert ArrayBuffer → Uint8Array → Vec<u8>
    let u8_array = Uint8Array::new(&abuf);
    let bytes = u8_array.to_vec();
    Ok(bytes)
}

#[derive(Debug, Clone)]
pub struct TextLayerConfig {
    preserve_text_formatting: bool,
    collect_words: bool,
    use_precise_char_bounds: bool,
    use_precise_font_size: bool,
}

impl Default for TextLayerConfig {
    fn default() -> Self {
        Self {
            preserve_text_formatting: true,
            collect_words: true,
            use_precise_char_bounds: false,
            use_precise_font_size: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DocumentViewerLayout {
    pub padding: u32,
    pub gap: u32,
    pub background: String,
    pub page_scale: f32,
}

impl DocumentViewerLayout {
    pub fn full_screen() -> Self {
        Self {
            padding: 0,
            gap: 0,
            background: "transparent".to_string(),
            page_scale: 1f32,
        }
    }
}

impl Default for DocumentViewerLayout {
    fn default() -> Self {
        Self {
            padding: 20,
            gap: 20,
            background: "gray".to_string(),
            page_scale: 0.5,
        }
    }
}

// TODO: adjust font size for scale
fn create_text_fragments(
    config: &TextLayerConfig,
    text: &PdfPageText,
    page_scale: f32,
) -> Vec<PdfText> {
    let words: Vec<PdfText> = text
        .chars()
        .iter()
        .filter_map(|c| {
            c.unicode_string().map(|s| {
                let bounds = if config.use_precise_char_bounds {
                    c.tight_bounds()
                } else {
                    c.loose_bounds()
                }
                .expect("bounds should be accessible");
                let bounds = PdfRect::new_from_values(
                    bounds.bottom().value * page_scale,
                    bounds.left().value * page_scale,
                    bounds.top().value * page_scale,
                    bounds.right().value * page_scale,
                );
                PdfText {
                    text: s.clone(),
                    font_family: c
                        .text_object()
                        .map(|t| t.font().family())
                        .unwrap_or_default(),
                    font_size: format!(
                        "{}pt",
                        (if config.use_precise_font_size {
                            c.scaled_font_size()
                        } else {
                            c.unscaled_font_size()
                        })
                        .value
                            * page_scale
                    ),
                    bounds: bounds,
                }
            })
        })
        .collect();
    words
}

// TODO: change parameter to bytes and then create a higher-level PDFViewer for fetching
#[component]
pub fn PdfDocument<FalFn, Fal>(
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
    /// Options for fetching the PDF file
    #[prop(default=RequestMode::SameOrigin)]
    mode: RequestMode,
    /// Configuration options for the selectable text layer. Not providing a value will not render a text layer
    #[prop(optional, into)]
    text_layer_config: MaybeProp<TextLayerConfig>,
    #[prop(optional, into)] set_captured_document_text: Option<WriteSignal<Vec<String>>>,
    #[prop(optional, into)] viewer_layout: Signal<DocumentViewerLayout>,
    #[prop(default=true.into(), into)] scrollbar: Signal<bool>,
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
    let pdf_data =
        LocalResource::new(move || async move { fetch_pdf_bytes(&url.get(), mode).await });
    let padding = format!("{}px", viewer_layout.get().padding);
    let gap = format!("{}px", viewer_layout.get().gap);
    let pdf_document_ref = NodeRef::<Div>::new();
    let UseElementSizeReturn { width, .. } = use_element_size(pdf_document_ref);
    view! {
        <div
            class="leptos-pdf-document"
            style:background=viewer_layout.get().background
            style:padding=padding
            style:gap=gap
            style:overflow_y=if scrollbar.get() { "auto" } else { "hidden" }
            node_ref=pdf_document_ref
        >
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
                        let mut captured_document_text: Vec<String> = Vec::new();
                        let scaled_width = (width.get() as f32 * viewer_layout.get().page_scale)
                            .round() as i32;
                        for page in pdf.pages().iter() {
                            let text_fragments: Vec<PdfText> = if let Some(text_layer_config) = text_layer_config
                                .get()
                            {
                                create_text_fragments(
                                    &text_layer_config,
                                    &page.text().expect("page text should be extractable"),
                                    1f32 + viewer_layout.get().page_scale,
                                )
                            } else {
                                vec![]
                            };
                            if set_captured_document_text.is_some() {
                                captured_document_text
                                    .push(
                                        page.text().expect("page text should be extractable").all(),
                                    );
                            }
                            let rendered_page = page
                                .render_with_config(
                                    &PdfRenderConfig::new().set_target_width(scaled_width),
                                )
                                .map_err(|e| PdfError::RenderError(format!("{}", e)))?;
                            let pixels: Vec<_> = rendered_page.as_rgba_bytes().to_vec();
                            let width = rendered_page.width() as u32;
                            let height = rendered_page.height() as u32;
                            views
                                .push(

                                    // Workaround by creating an ImageData in Rust
                                    // Issue: https://github.com/wasm-bindgen/wasm-bindgen/issues/4815
                                    // asdf
                                    // render
                                    view! {
                                        <PdfPage
                                            pixels
                                            width
                                            height
                                            text_fragments=text_fragments
                                        />
                                    }
                                        .into_any(),
                                );
                        }
                        if let Some(set_captured_document_text) = set_captured_document_text {
                            set_captured_document_text.set(captured_document_text);
                        }
                        Ok(views)
                    })}
                </Transition>
            </ErrorBoundary>
        </div>
    }
}
