use leptos::prelude::*;
use pdfium_render::prelude::{PdfPageTextSegment, PdfPoints, PdfRect};

struct TextParagraph {
    lines: Vec<TextLine>,
}

struct TextLine {
    words: Vec<TextWord>,
}

pub struct TextWord {
    text: String,
    bounds: PdfRect,
}

impl From<PdfPageTextSegment<'_>> for TextWord {
    fn from(value: PdfPageTextSegment) -> Self {
        Self {
            text: value.text(),
            bounds: value.bounds(),
        }
    }
}

#[component]
pub fn PdfPage(#[prop(optional)] words: Vec<TextWord>) -> impl IntoView {
    view! { <div>"PDF Page Component"</div> }
}
