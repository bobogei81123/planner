import { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
  schema: 'http://localhost:8000/graphql',
  documents: ['src/**/*.svelte', 'src/**/*.ts', 'src/route/+layout.svelte'],
  generates: {
    './src/gql/': {
      preset: 'client',
      config: {
        useTypeImports: true
      }
    },
    './src/generated-introspection.json': {
      plugins: ['urql-introspection'],
    }
  }
};

export default config;
