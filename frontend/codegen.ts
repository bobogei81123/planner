import { CodegenConfig } from '@graphql-codegen/cli'

const config: CodegenConfig = {
  schema: 'http://localhost:8000/graphql',
  documents: ['src/**/*.svelte', 'src/**/*.ts'],
  generates: {
    './src/gql/': {
      preset: 'client',
      config: {
        useTypeImports: true
      },
    },
  },
};

export default config;
