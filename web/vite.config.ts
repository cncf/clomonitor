import { fileURLToPath, URL } from 'node:url';

import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '~': fileURLToPath(new URL('./node_modules/', import.meta.url)),
    },
  },
  css: {
    preprocessorOptions: {
      scss: {
        quietDeps: true,
        silenceDeprecations: ['import'],
        importers: [
          {
            findFileUrl(url: string) {
              if (!url.startsWith('~')) {
                return null;
              }
              return new URL(`./node_modules/${url.slice(1)}`, import.meta.url);
            },
          },
        ],
      },
    },
  },
  server: {
    host: true,
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:8000',
        changeOrigin: true,
      },
    },
  },
  preview: {
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:8000',
        changeOrigin: true,
      },
    },
  },
  build: {
    outDir: 'build',
    assetsDir: 'static',
    emptyOutDir: true,
    sourcemap: true,
    rollupOptions: {
      output: {
        entryFileNames: (chunkInfo) =>
          chunkInfo.name === 'index'
            ? 'static/js/main.[hash].js'
            : 'static/js/[name].[hash].js',
        chunkFileNames: 'static/js/[name].[hash].js',
        manualChunks: (id) => {
          if (id.includes('node_modules')) {
            if (id.includes('react')) {
              return 'vendor-react';
            }
            if (id.includes('clo-ui')) {
              return 'vendor-cloui';
            }
            if (id.includes('lodash')) {
              return 'vendor-lodash';
            }
            if (id.includes('moment')) {
              return 'vendor-moment';
            }
            if (id.includes('apexcharts') || id.includes('react-apexcharts')) {
              return 'vendor-apexcharts';
            }
            return 'vendor';
          }
          return undefined;
        },
        assetFileNames: (assetInfo) => {
          if (assetInfo.name?.endsWith('.css')) {
            return 'static/css/main.[hash][extname]';
          }
          return 'static/media/[name].[hash][extname]';
        },
      },
    },
  },
  test: {
    environment: 'jsdom',
    setupFiles: './src/setupTests.ts',
    globals: true,
    css: true,
  },
});
