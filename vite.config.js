import { defineConfig } from "vite";

export default defineConfig({
    publicDir: false,
    build: {
        outDir: "bundled_javascript",      // where to emit your bundled file
        lib: {
            entry: "./javascript/pdf_glue.js",
            formats: ["es"],
            fileName: "pdf_glue.bundle"
        },
        rollupOptions: {
            output: {
                // ensures proper path for worker assets
                assetFileNames: "assets/[name].[ext]"
            }
        }
    }
});
