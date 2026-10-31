import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Tauri expects a fixed port and doesn't want Vite to clear its error output.
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: { port: 1420, strictPort: true, watch: { ignored: ["**/src-tauri/**", "**/content/**"] } },
  build: { target: "es2022", chunkSizeWarningLimit: 2000 },
});
