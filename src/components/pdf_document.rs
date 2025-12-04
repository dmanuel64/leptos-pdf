use leptos::{html::Div, prelude::*};
use leptos_use::{use_element_size, UseElementSizeReturn};
use pdfium_render::prelude::*;

use crate::{
    components::{
        pdf_page::{PdfTextLine, PdfTextWord},
        PdfPage,
    },
    errors::PdfError,
};

fn create_lines(
    text: &PdfPageText,
    page_zoom: f32,
    use_precise_font_size: bool,
    use_precise_char_bounds: bool,
    use_precise_line_bounds: bool,
) -> Vec<PdfTextLine> {
    let lines = text.all();
    let lines = lines.split("\n");
    let char_iter = text.chars().iter();
    for line in lines {
        let words = line.split_whitespace();
        for word in words {}
    }
    let words: Vec<PdfTextWord> = text
        .chars()
        .iter()
        .filter_map(|c| {
            c.unicode_string().map(|s| {
                let bounds = if use_precise_char_bounds {
                    c.tight_bounds()
                } else {
                    c.loose_bounds()
                }
                .expect("bounds should be accessible");
                let bounds = PdfRect::new_from_values(
                    bounds.bottom().value * page_zoom,
                    bounds.left().value * page_zoom,
                    bounds.top().value * page_zoom,
                    bounds.right().value * page_zoom,
                );
                PdfTextWord {
                    text: s.clone(),
                    font_family: c
                        .text_object()
                        .map(|t| t.font().family())
                        .unwrap_or_default(),
                    font_size: format!(
                        "{}pt",
                        (if use_precise_font_size {
                            c.scaled_font_size()
                        } else {
                            c.unscaled_font_size()
                        })
                        .value
                            * page_zoom
                    ),
                    bounds: bounds,
                }
            })
        })
        .collect();
    vec![]
}

/// Component to render a PDF document from given bytes
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
    #[prop(default="20px".into(), into)] padding: Signal<String>,
    #[prop(default="20px".into(), into)] gap: Signal<String>,
    #[prop(default="gray".into(), into)] background: Signal<String>,
    #[prop(default=0.5.into(), into)] page_zoom: Signal<f32>,
    #[prop(default=true.into(), into)] show_scrollbar: Signal<bool>,
    #[prop(optional, into)] pdf_text: RwSignal<Option<Vec<String>>>,
    #[prop(optional, into)] enable_text_layer: Signal<bool>,
    #[prop(optional, into)] use_precise_font_size: Signal<bool>,
    #[prop(optional, into)] use_precise_char_bounds: Signal<bool>,
    #[prop(optional, into)] use_precise_line_bounds: Signal<bool>,
) -> impl IntoView
where
    FalFn: FnMut(ArcRwSignal<Errors>) -> Fal + Send + 'static,
    Fal: IntoView + Send + 'static,
{
    let pdf_document_ref = NodeRef::<Div>::new();
    let UseElementSizeReturn { width, .. } = use_element_size(pdf_document_ref);
    view! {
        <div
            class="leptos-pdf-document"
            style:background=background
            style:padding=padding
            style:gap=gap
            style:overflow_y=if show_scrollbar.get() { "auto" } else { "hidden" }
            node_ref=pdf_document_ref
        >
            <ErrorBoundary fallback=error_fallback>
                {move || -> Result<Vec<AnyView>, PdfError> {
                    let pdf = Pdfium::default();
                    let pdf = pdf
                        .load_pdf_from_byte_vec(pdf_bytes.get(), password.get().as_deref())
                        .map_err(|e| PdfError::LoadingError(format!("{}", e)))?;
                    let mut views: Vec<AnyView> = Vec::new();
                    let scaled_width = (width.get() as f32 * page_zoom.get()).round() as i32;
                    for page in pdf.pages().iter() {
                        let text_fragments: Vec<PdfTextWord> = if enable_text_layer.get() {
                            // create_lines(
                            //     &page.text().expect("page text should be extractable"),
                            //     1f32 + page_zoom.get(),
                            //     use_precise_font_size.get(),
                            //     use_precise_char_bounds.get(),
                            // )
                            vec![]
                        } else {
                            vec![]
                        };
                        pdf_text
                            .maybe_update(|s| {
                                if let Some(d) = s {
                                    d.push("".to_string());
                                    true
                                } else {
                                    false
                                }
                            });
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
                                view! {
                                    <PdfPage pixels width height text_fragments=text_fragments />
                                }
                                    .into_any(),
                            );
                    }
                    Ok(views)
                }}
            </ErrorBoundary>
        </div>
    }
}
