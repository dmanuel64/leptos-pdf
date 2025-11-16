use leptos::{html::Canvas, prelude::*};
use pdfium_render::prelude::{PdfPageTextSegment, PdfRect};

pub struct PageWord {
    text: String,
    bounds: PdfRect,
}

impl From<PdfPageTextSegment<'_>> for PageWord {
    fn from(value: PdfPageTextSegment) -> Self {
        Self {
            text: value.text(),
            bounds: value.bounds(),
        }
    }
}

#[component]
pub fn PdfPage(#[prop(optional)] words: Vec<PageWord>) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();
    view! {
        <div class="leptos-pdf-page">
            <canvas class="leptos-pdf-page-canvas" node_ref=canvas_ref />
        </div>
    }
}
