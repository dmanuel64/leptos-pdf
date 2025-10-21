use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/bundled_javascript/pdf_glue.bundle.mjs")]
extern "C" {
    #[derive(Debug, Clone)]
    pub type PdfHandle;

    #[wasm_bindgen(js_name = renderPage, method, catch)]
    pub async fn render_page(
        this: &PdfHandle,
        page_num: usize,
        scale: f32,
        text: bool,
        annotations: bool,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = loadPdf, catch)]
    async fn load_pdf_no_cast(
        canvas_id: &str,
        url: &str,
        text_layer_id: &str,
        annotation_layer_id: &str,
    ) -> Result<JsValue, JsValue>;
}

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
