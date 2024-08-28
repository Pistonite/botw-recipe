import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import yaml from "@modyfi/vite-plugin-yaml";
import tsconfigPaths from "vite-tsconfig-paths";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    react(),
    yaml(),
    tsconfigPaths(),
  ],
  build: {
    rollupOptions: {
      input: {
        tauri: "index-tauri.html",
      }
    }
  },

  // prevent vite from obscuring rust errors when running from tauri
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
