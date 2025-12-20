import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],

  // Tauri expects a fixed port
  server: {
    port: 3000,
    strictPort: true,
  },

  // Prevent vite from obscuring rust errors
  clearScreen: false,

  // Tauri root is the src-tauri folder
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  },

  // Vitest configuration
  test: {
    globals: true,
    environment: 'happy-dom',
    setupFiles: ['./src/test/setup.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      exclude: [
        'node_modules/',
        'src/test/',
        '**/*.spec.ts',
        '**/*.test.ts',
      ]
    }
  }
})

