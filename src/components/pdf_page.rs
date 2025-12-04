use leptos::{html::Canvas, prelude::*};
use pdfium_render::prelude::PdfRect;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, ImageData};

// TODO: this would probably make more sense as a slot
// TODO: make this similar in DocumentViewerLayout, use u32 and format to px when applying styles
#[derive(Debug, Clone)]
pub struct PdfText {
    pub text: String,
    pub font_family: String,
    /// The size of the font as a valid
    /// [`<length>`](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/length#syntax)
    /// CSS data type in absolute length units
    pub font_size: String,
    pub bounds: PdfRect,
}

// TODO: text_fragments should probably not be a signal if most other props in this component are not reactive
#[component]
pub fn PdfPage(
    pixels: Vec<u8>,
    width: u32,
    height: u32,
    text_fragments: Vec<PdfText>,
) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();
    let canvas_width = format!("{width}px");
    let canvas_height = format!("{height}px");
    let text_layer_height = canvas_height.clone();
    Effect::new(move |_| {
        if let Some(canvas_ref) = canvas_ref.get() {
            let ctx = canvas_ref
                .get_context("2d")
                .expect("2d should be a valid Canvas context identifier")
                .expect("there should be a 2d context attached to the canvas")
                .dyn_into::<CanvasRenderingContext2d>()
                .expect("the 2d context should be of type CanvasRenderingContext2d");
            // log::warn!("{:?}", rendered_page.data());
            let rendered_page =
                ImageData::new_with_u8_clamped_array_and_sh(Clamped(&pixels), width, height)
                    .expect("ImageData to be created without any errors");
            ctx.put_image_data(&rendered_page, 0f64, 0f64)
                .expect("put_image_data should not raise NotSupportedError or InvalidStateError");
        }
    });
    let no_text = text_fragments.is_empty();
    view! {
        <div class="leptos-pdf-page" style:position="relative" style:width=canvas_width.clone() style:height=canvas_height.clone()>
            <Show when=move || !no_text>
                <div class="leptos-pdf-text-layer" >
                    {text_fragments
                        .iter()
                        .map(|t| {
                            let left = format!("{}px", t.bounds.left().value);
                            let top = format!("{}px", t.bounds.top().value);
                            view! {
                                <span
                                    class="leptos-pdf-text-fragment"
                                    style:font-size=t.font_size.clone()
                                    style:left=left
                                    style:top=format!("calc({} - {top})", text_layer_height)
                                    style:font-family=t.font_family.clone()
                                >
                                    {t.text.clone()}
                                </span>
                            }
                        })
                        .collect_view()}
                </div>
            </Show>
            <canvas
                class="leptos-pdf-page-canvas"
                node_ref=canvas_ref
                width=canvas_width.clone()
                height=canvas_height.clone()
            />
        </div>
    }
}
