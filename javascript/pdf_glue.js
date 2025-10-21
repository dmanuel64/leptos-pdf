import * as pdfjsLib from "pdfjs-dist/build/pdf.mjs";
import pdfjsWorker from "pdfjs-dist/build/pdf.worker.mjs?url";

pdfjsLib.GlobalWorkerOptions.workerSrc = pdfjsWorker;
export class PdfHandle {
  constructor(pdfProxy, canvasId, textLayerId) {
    this.pdfProxy = pdfProxy;
    this.canvasId = canvasId;
    this.textLayerId = textLayerId
  }

  async renderPage(pageNumber, scale = 1.0, text = false) {
    const page = await this.pdfProxy.getPage(pageNumber);
    const viewport = page.getViewport({ scale });
    const canvas = document.getElementById(this.canvasId);
    const textLayer = document.getElementById(this.textLayerId)
    const ctx = canvas.getContext('2d')
    canvas.width = viewport.width;
    canvas.height = viewport.height;
    await page.render({ canvasContext: ctx, viewport }).promise
    if (text && textLayer !== null) {
      page.getTextContent().then((content) => {
        textLayer.style.setProperty("--total-scale-factor", `${scale}`);
        textLayer.style.setProperty("--scale-factor", `${scale}`);
        textLayer.style.setProperty("--scale-round-x", "1px");
        textLayer.style.setProperty("--scale-round-y", "1px");
        const renderTask = new pdfjsLib.TextLayer({
          container: textLayer,
          textContentSource: content,
          viewport: viewport.clone({ dontFlip: true })
        });
        renderTask.render();
      });
    }
  }

}

export async function loadPdf(canvas_id, url, textLayerId = null) {
  const resp = await fetch(url);
  const data = new Uint8Array(await resp.arrayBuffer());
  const loadingTask = pdfjsLib.getDocument({ data });
  const pdfProxy = await loadingTask.promise;
  const pdfViewer = new PdfHandle(pdfProxy, canvas_id, textLayerId);
  return pdfViewer;
}
