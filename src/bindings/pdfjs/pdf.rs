//! Bindings for the PDF.js PDF rendering bridge.

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/bundled_javascript/pdf_glue.bundle.mjs")]
extern "C" {
    /// A reference to a `PDFHandle` class
    #[derive(Debug, Clone)]
    pub type PdfHandle;

    /// Renders a page of the loaded PDF document into an associated `canvas`.
    ///
    /// # Arguments
    ///
    /// * `page_num` - The 1-based page index to render.
    /// * `scale`: The scale factor to render the page at.
    /// * `text`: Whether to render the text layer on top of the canvas.
    /// * `annotations`: Whether to render annotations on top of the text layer.
    #[wasm_bindgen(js_name = renderPage, method, catch)]
    pub async fn render_page(
        this: &PdfHandle,
        page_num: usize,
        scale: f32,
        text: bool,
        annotations: bool,
    ) -> Result<(), JsValue>;

    /// Loads a PDF document and returns a [`PdfHandle`] for further interaction.
    ///
    /// # Arguments
    ///
    /// * `canvas_id`: The DOM element ID of the target `canvas` for rendering.
    /// * `url`: The URL of the PDF document to load.
    /// * `text_layer_id`: The DOM element ID of the container for selectable text content.
    /// * `annotation_layer_id`: The DOM element ID ID of the container for annotations.
    #[wasm_bindgen(js_name = loadPdf, catch)]
    async fn load_pdf_no_cast(
        canvas_id: &str,
        url: &str,
        text_layer_id: &str,
        annotation_layer_id: &str,
    ) -> Result<JsValue, JsValue>;
}

/// A safe Rust wrapper around the low-level [`load_pdf_no_cast`]
pub async fn load_pdf(
    canvas_id: &str,
    url: &str,
    text_layer_id: &str,
    annotation_layer_id: &str,
) -> Result<PdfHandle, JsValue> {
    let pdf_viewer = PdfHandle::from(
        load_pdf_no_cast(canvas_id, url, text_layer_id, annotation_layer_id).await?,
    );
    Ok(pdf_viewer)
}
