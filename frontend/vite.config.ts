import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import codegen from 'vite-plugin-graphql-codegen';

export default defineConfig({
  plugins: [codegen(), sveltekit()],
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}']
  },
  server: {
    proxy: {
      '/graphql': 'http://localhost:8000',
    }
  }
});
