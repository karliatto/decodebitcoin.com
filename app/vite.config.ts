import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import path from 'path';

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
  resolve: {
    alias: {
      'env': false as any,
      '@pkg': path.resolve(__dirname, '../pkg'),
    },
  },
  server: {
    fs: {
      // Allow serving files from the pkg directory outside of root
      allow: ['..', '../pkg'],
    },
  },
})
