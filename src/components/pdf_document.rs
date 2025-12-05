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

#[derive(Debug)]
struct CharacterMapping {
    text: char,
    font_family: String,
    /// Font size in points
    font_size: f32,
    bounds: PdfRect,
}

#[derive(Debug, Clone)]
pub enum FontSizeMatch {
    /// Only identical font sizes are grouped (tolerance = 0px)
    Strict,
    /// Font sizes may differ by up to this many points
    Tolerant(u32),
    /// Font size is ignored — always considered part of the same word
    Any,
}

impl Default for FontSizeMatch {
    fn default() -> Self {
        FontSizeMatch::Tolerant(5)
    }
}

impl CharacterMapping {
    pub fn create_words<CharIter>(
        word_chars: CharIter,
        font_size_match: &FontSizeMatch,
        require_same_font: bool,
    ) -> Vec<PdfTextWord>
    where
        CharIter: Iterator<Item = CharacterMapping>,
    {
        let mut words: Vec<PdfTextWord> = Vec::new();
        for word_char in word_chars {
            if let Some(current_word) = words.last_mut() {
                if !require_same_font || word_char.font_family == current_word.font_family {
                    let current_word_font_size = current_word.font_size;
                    let font_size_matches = match font_size_match {
                        FontSizeMatch::Strict => {
                            word_char.font_size.round() == current_word_font_size.round()
                        }
                        FontSizeMatch::Tolerant(tolerance) => {
                            (word_char.font_size - current_word_font_size).abs()
                                <= *tolerance as f32
                        }
                        FontSizeMatch::Any => true,
                    };
                    if font_size_matches {
                        current_word.text.push_str(&word_char.text.to_string());
                        current_word.bounds = PdfRect::new_from_values(
                            current_word
                                .bounds
                                .bottom()
                                .value
                                .min(word_char.bounds.bottom().value),
                            current_word
                                .bounds
                                .left()
                                .value
                                .min(word_char.bounds.left().value),
                            current_word
                                .bounds
                                .top()
                                .value
                                .max(word_char.bounds.top().value),
                            current_word
                                .bounds
                                .right()
                                .value
                                .max(word_char.bounds.right().value),
                        );
                        continue;
                    }
                }
            }
            words.push(PdfTextWord {
                text: word_char.text.to_string(),
                font_family: word_char.font_family,
                font_size: word_char.font_size,
                bounds: word_char.bounds,
            });
        }
        words
    }
}

fn create_lines(
    text: &PdfPageText,
    page_zoom: f32,
    use_precise_font_size: bool,
    use_precise_char_bounds: bool,
    use_precise_line_bounds: bool,
    font_size_match: &FontSizeMatch,
    require_same_font: bool,
) -> Vec<PdfTextWord> {
    let mut w: Vec<PdfTextWord> = Vec::new();
    let lines = text.all();
    let lines = lines.split("\n");
    let char_iter = text.chars();
    let mut char_iter = char_iter.iter();
    for line in lines {
        let words = line.split_whitespace();
        for word in words {
            let mut char_bounds_mapping: Vec<CharacterMapping> = Vec::new();
            for word_char in word.chars() {
                if let Some(page_char) = char_iter.find(|c| {
                    c.unicode_char()
                        .map(|uc| uc == word_char)
                        .unwrap_or_default()
                }) {
                    char_bounds_mapping.push(CharacterMapping {
                        text: word_char,
                        bounds: if use_precise_char_bounds {
                            page_char.tight_bounds()
                        } else {
                            page_char.loose_bounds()
                        }
                        .expect("PDF text bounds should be accessible"),
                        font_family: page_char
                            .text_object()
                            .map(|t| t.font().family())
                            .unwrap_or_default(),
                        font_size: ((if use_precise_font_size {
                            page_char.scaled_font_size()
                        } else {
                            page_char.unscaled_font_size()
                        })
                        .value
                            * page_zoom),
                    });
                } else {
                    log::warn!("Could not find page character for '{word_char}'");
                }
            }
            let pdf_words = CharacterMapping::create_words(
                char_bounds_mapping.into_iter(),
                font_size_match,
                require_same_font,
            );
            w.extend(pdf_words);
        }
    }
    w
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
    #[prop(optional, into)] font_size_match: Signal<FontSizeMatch>,
    #[prop(default=false.into(), into)] require_same_font: Signal<bool>,
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
                            create_lines(
                                &page
                                    .text()
                                    .map_err(|e| PdfError::TextExtractionError(format!("{}", e)))?,
                                page_zoom.get(),
                                use_precise_font_size.get(),
                                use_precise_char_bounds.get(),
                                use_precise_line_bounds.get(),
                                &font_size_match.get(),
                                require_same_font.get(),
                            )
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
                                // create_lines(
                                // &page.text().expect("page text should be extractable"),
                                // 1f32 + page_zoom.get(),
                                // use_precise_font_size.get(),
                                // use_precise_char_bounds.get(),
                                // )
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
