import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import codegen from 'vite-plugin-graphql-codegen';

export default defineConfig({
  plugins: [codegen({ configOverride: { noSilentErrors: true }, runOnBuild: false }), sveltekit()],
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}']
  },
  server: {
    proxy: {
      '/graphql': 'http://127.0.0.1:8000',
      '/auth': 'http://127.0.0.1:8000'
    }
  }
});
