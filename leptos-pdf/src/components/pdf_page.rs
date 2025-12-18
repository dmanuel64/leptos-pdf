//! Low-level PDF page rendering logic and the [`PdfPage`] component.

use leptos::{html::Canvas, prelude::*};
use pdfium_render::prelude::PdfRect;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, ImageData};

// TODO: this would probably make more sense as a slot
/// Represents a single word extracted from a PDF page.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PdfTextWord {
    /// The textual content of the word.
    pub text: String,
    /// The font family used to render the word.
    pub font_family: String,
    /// The font size of the word, expressed in points.
    pub font_size: f32,
    /// The bounding box of the word PDF points.
    pub bounds: PdfRect,
}

// #[derive(Debug, Clone)]
// pub struct PdfTextLine {
//     pub words: Vec<PdfTextWord>,
// }

/// A component that renders a single page of a PDF document.
///
/// The page is rendered in two layers:
///
/// 1. **Canvas layer**: Displays the rasterized PDF page using raw RGBA pixel data.
/// 2. **Text layer (optional)**: Renders positioned `<span>` elements over the canvas for
///     selectable and searchable text.
#[component]
pub fn PdfPage(
    /// Raw RGBA pixel data for the rendered page image.
    pixels: Vec<u8>,
    /// Width of the page in pixels.
    width: u32,
    /// Height of the page in pixels.
    height: u32,
    // Extracted text words to render in the text overlay. If empty, the text layer is not
    // rendered.
    // words: Vec<PdfTextWord>,
) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();
    let canvas_width = format!("{width}px");
    let canvas_height = format!("{height}px");
    // let text_layer_height = canvas_height.clone();
    Effect::new(move |_| {
        // Render the pixel buffer into the canvas once it becomes available
        if let Some(canvas_ref) = canvas_ref.get() {
            let ctx = canvas_ref
                .get_context("2d")
                .expect("2d should be a valid Canvas context identifier")
                .expect("there should be a 2d context attached to the canvas")
                .dyn_into::<CanvasRenderingContext2d>()
                .expect("the 2d context should be of type CanvasRenderingContext2d");
            let rendered_page =
                ImageData::new_with_u8_clamped_array_and_sh(Clamped(&pixels), width, height)
                    .expect("ImageData to be created without any errors");
            ctx.put_image_data(&rendered_page, 0f64, 0f64)
                .expect("put_image_data should not raise NotSupportedError or InvalidStateError");
        }
    });
    // let no_text = words.is_empty();
    view! {
        <div class="leptos-pdf-page" style:position="relative" style:width=canvas_width.clone() style:height=canvas_height.clone()>
            // <Show when=move || !no_text>
            //     <div class="leptos-pdf-text-layer" >
            //         {words
            //             .iter()
            //             .map(|t| {
            //                 let left = format!("{}px", t.bounds.left().value);
            //                 let top = format!("{}px", t.bounds.top().value);
            //                 view! {
            //                     <span
            //                         class="leptos-pdf-text-fragment"
            //                         style:font-size=format!("{}pt", t.font_size.clone())
            //                         style:left=left
            //                         style:top=format!("calc({} - {top})", text_layer_height)
            //                         style:font-family=t.font_family.clone()
            //                     >
            //                         {t.text.clone()}
            //                     </span>
            //                 }
            //             })
            //             .collect_view()}
            //     </div>
            // </Show>
            <canvas
                class="leptos-pdf-page-canvas"
                node_ref=canvas_ref
                width=canvas_width.clone()
                height=canvas_height.clone()
            />
        </div>
    }
}
