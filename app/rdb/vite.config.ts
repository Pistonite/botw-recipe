import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import yaml from "@modyfi/vite-plugin-yaml";
import tsconfigPaths from "vite-tsconfig-paths";
import griffel from "@griffel/vite-plugin";

// https://vitejs.dev/config/
export default defineConfig(({ command }) => ({
  plugins: [
    react(),
    yaml(),
    tsconfigPaths(),
        command === "build" && griffel(),
  ],
    json: {
        // note: doesn't work on yaml
        // see https://github.com/Modyfi/vite-plugin-yaml/issues/30
        stringify: true,
    },
  build: {
    rollupOptions: {
      input: {
        tauri: "index-tauri.html",
      }
    },
        cssCodeSplit: false,
        chunkSizeWarningLimit: 1024,
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
