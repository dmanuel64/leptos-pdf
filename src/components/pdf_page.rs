use image::DynamicImage;
use leptos::{html::Canvas, prelude::*};
use pdfium_render::prelude::{PdfPageTextSegment, PdfRect};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, ImageData};

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
pub fn PdfPage(rendered_page: ImageData, #[prop(optional)] words: Vec<PageWord>) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();
    Effect::new(move |_| {
        if let Some(canvas_ref) = canvas_ref.get() {
            let ctx = canvas_ref
                .get_context("2d")
                .expect("A canvas 2D context should be available")
                .expect("There should be a 2D canvas context")
                .dyn_into::<CanvasRenderingContext2d>()
                .expect("The 2D context should be of type CanvasRenderingContext2d");
            ctx.put_image_data(&rendered_page, 0f64, 0f64);
            log::warn!("Canvas was loaded");
        }
    });
    view! {
        <div class="leptos-pdf-page">
            <canvas class="leptos-pdf-page-canvas" node_ref=canvas_ref />
        </div>
    }
}
