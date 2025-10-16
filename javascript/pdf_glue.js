import * as pdfjsLib from "pdfjs-dist/build/pdf.mjs";
import pdfjsWorker from "pdfjs-dist/build/pdf.worker.mjs?url";

pdfjsLib.GlobalWorkerOptions.workerSrc = pdfjsWorker;

let pdfDoc = null;
let currentPage = 1;

export async function load_pdf(canvas_id, url) {
  const resp = await fetch(url);
  const data = new Uint8Array(await resp.arrayBuffer());
  const loadingTask = pdfjsLib.getDocument({ data });
  pdfDoc = await loadingTask.promise;
  await render_page(canvas_id, currentPage);
  return pdfDoc.numPages;
}

export async function render_page(canvas_id, page_num) {
  if (!pdfDoc) return;
  currentPage = page_num;

  const page = await pdfDoc.getPage(page_num);
  const viewport = page.getViewport({ scale: 1.5 });
  const canvas = document.getElementById(canvas_id);
  const ctx = canvas.getContext("2d");
  canvas.width = viewport.width;
  canvas.height = viewport.height;

  await page.render({ canvasContext: ctx, viewport }).promise;
}

export async function next_page(canvas_id) {
  if (currentPage < pdfDoc.numPages) {
    currentPage++;
    await render_page(canvas_id, currentPage);
  }
  return currentPage;
}

export async function prev_page(canvas_id) {
  if (currentPage > 1) {
    currentPage--;
    await render_page(canvas_id, currentPage);
  }
  return currentPage;
}
