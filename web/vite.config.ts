import { fileURLToPath, URL } from 'node:url';

import type { Plugin } from 'vite';
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

const isTest = Boolean(process.env.VITEST);

const removeApexchartsPreload = (): Plugin => {
  return {
    name: 'remove-apexcharts-preload',
    apply: 'build',
    transformIndexHtml(html: string) {
      return html.replace(
        /\s*<link rel="modulepreload"[^>]*static\/js\/vendor-apexcharts\.[^>]+>\s*/g,
        ''
      );
    },
  };
};

export default defineConfig({
  plugins: [react(), removeApexchartsPreload()],
  resolve: {
    alias: {
      '~': fileURLToPath(new URL('./node_modules/', import.meta.url)),
    },
  },
  css: {
    modules: {
      generateScopedName: isTest ? '[local]' : undefined,
    },
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
      '/data': {
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
          if (!id.includes('node_modules')) {
            return undefined;
          }
          if (id.includes('node_modules/clo-ui')) {
            return 'vendor-cloui';
          }
          if (id.includes('lodash')) {
            return 'vendor-lodash';
          }
          if (id.includes('date-fns')) {
            return 'vendor-datefns';
          }
          if (id.includes('apexcharts') || id.includes('react-apexcharts')) {
            return 'vendor-apexcharts';
          }
          return 'vendor';
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
