use leptos::{html::Div, prelude::*};
use leptos_use::{UseElementSizeReturn, use_element_size};
use pdfium_render::prelude::*;

use crate::{
    components::{PdfPage, pdf_page::PdfText},
    errors::PdfError,
};

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
    /// PDF data
    #[prop(into)]
    pdf_bytes: Signal<Vec<u8>>,
    /// Password to access the PDF if required
    #[prop(optional, into)]
    password: MaybeProp<String>,
    /// View to display if an error is encountered and the PDF cannot be loaded
    error_fallback: FalFn,
    /// Configuration options for the selectable text layer. Not providing a value will not render a text layer
    #[prop(optional, into)]
    text_layer_config: MaybeProp<TextLayerConfig>,
    /// Optional signal to capture all text in the document
    #[prop(optional, into)]
    set_captured_document_text: Option<WriteSignal<Vec<String>>>,
    /// Layout options for the document viewer
    #[prop(optional, into)]
    viewer_layout: Signal<DocumentViewerLayout>,
    /// Whether to show a scrollbar when the content overflows
    #[prop(default=true.into(), into)]
    scrollbar: Signal<bool>,
) -> impl IntoView
where
    FalFn: FnMut(ArcRwSignal<Errors>) -> Fal + Send + 'static,
    Fal: IntoView + Send + 'static,
{
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
                {move || -> Result<Vec<AnyView>, PdfError> {
                    let pdf = Pdfium::default();
                    let pdf = pdf
                        .load_pdf_from_byte_vec(pdf_bytes.get(), password.get().as_deref())
                        .map_err(|e| PdfError::LoadingError(format!("{}", e)))?;
                    let mut views: Vec<AnyView> = Vec::new();
                    let mut captured_document_text: Vec<String> = Vec::new();
                    let scaled_width = (width.get() as f32 * viewer_layout.get().page_scale).round()
                        as i32;
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
                                .push(page.text().expect("page text should be extractable").all());
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
                                    <PdfPage pixels width height text_fragments=text_fragments />
                                }
                                    .into_any(),
                            );
                    }
                    if let Some(set_captured_document_text) = set_captured_document_text {
                        set_captured_document_text.set(captured_document_text);
                    }
                    Ok(views)
                }}
            </ErrorBoundary>
        </div>
    }
}
