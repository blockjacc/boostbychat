// vite.config.ts
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { resolve } from 'path';

export default defineConfig({
  plugins: [vue()],

  /* vite-optimize for the Buffer shim */
  optimizeDeps: { include: ['buffer'] },

  /* -------- alias section -------- */
  resolve: {
    alias: {
      '@':     resolve(__dirname, 'src'), // import '@/foo' → src/foo
      buffer:  'buffer',                  // keep Buffer poly-fill happy
    },
  },

  /* some libs expect a global obj in the browser */
  define: { global: 'window' },

  /* proxy any /api/* XHR from :5173 → :3001 */
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:3001',
        changeOrigin: true,
      },
    },
  },
});
