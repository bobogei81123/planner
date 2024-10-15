import path from "path"
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import codegen from 'vite-plugin-graphql-codegen';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), codegen()],
  server: {
    proxy: {
      '/graphql': 'http://127.0.0.1:8000',
      '/auth': 'http://127.0.0.1:8000'
    }
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
})
