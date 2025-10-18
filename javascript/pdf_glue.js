import * as pdfjsLib from "pdfjs-dist/build/pdf.mjs";
import pdfjsWorker from "pdfjs-dist/build/pdf.worker.mjs?url";

pdfjsLib.GlobalWorkerOptions.workerSrc = pdfjsWorker;
export class PdfHandle {
  constructor(pdfProxy, canvasId) {
    this.pdfProxy = pdfProxy;
    this.canvasId = canvasId;
  }

  async renderPage(pageNumber, scale = 1.0) {
    const page = await this.pdfProxy.getPage(pageNumber);
    const viewport = page.getViewport({ scale });
    const canvas = document.getElementById(this.canvasId);
    const ctx = canvas.getContext('2d')
    canvas.width = viewport.width;
    canvas.height = viewport.height;
    await page.render({ canvasContext: ctx, viewport }).promise
  }

}

export async function loadPdf(canvas_id, url) {
  const resp = await fetch(url);
  const data = new Uint8Array(await resp.arrayBuffer());
  const loadingTask = pdfjsLib.getDocument({ data });
  const pdfProxy = await loadingTask.promise;
  const pdfViewer = new PdfHandle(pdfProxy, canvas_id);
  return pdfViewer;
}
