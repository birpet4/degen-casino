import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import rollupNodePolyfills from 'rollup-plugin-node-polyfills';
import inject from '@rollup/plugin-inject';
import { nodePolyfills } from 'vite-plugin-node-polyfills';

export default defineConfig({
  plugins: [react(),
    nodePolyfills({
      globals: {
        Buffer: true,
        global: true,
        process: true,
      },
  }),],
  server: {
    host: true, // This allows Vite to be accessible over the network
    port: 5173, // Default port, can be changed if necessary
  },
  resolve: {
    alias: {
      process: 'process/browser',
      stream: 'stream-browserify',
      zlib: 'browserify-zlib',
      util: 'util',
    },
  },
})
