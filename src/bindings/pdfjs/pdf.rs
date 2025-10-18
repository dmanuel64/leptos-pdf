use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/bundled_javascript/pdf_glue.bundle.mjs")]
extern "C" {
    #[derive(Debug, Clone)]
    pub type PdfHandle;

    #[wasm_bindgen(js_name = renderPage, method, catch)]
    pub async fn render_page(this: &PdfHandle, page_num: usize) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = loadPdf, catch)]
    async fn load_pdf_no_cast(canvas_id: &str, url: &str) -> Result<JsValue, JsValue>;
}

pub async fn load_pdf(canvas_id: &str, url: &str) -> Result<PdfHandle, JsValue> {
    let pdf_viewer = PdfHandle::from(load_pdf_no_cast(canvas_id, url).await?);
    Ok(pdf_viewer)
}
