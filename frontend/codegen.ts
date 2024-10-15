import { CodegenConfig } from '@graphql-codegen/cli';

const generates: CodegenConfig['generates'] = {
  './src/graphql/generated/': {
    preset: 'client',
    presetConfig: {
      gqlTagName: 'gql',
    },
    plugins: [],
    config: {
      scalars: {
        NaiveDate: 'string',
        UUID: 'string',
      },
    },
  },
};
if (process.env.NODE_ENV !== 'production') {
  generates['./schema.graphql'] = {
    plugins: ['schema-ast'],
  };
}

const config: CodegenConfig = {
  schema:
    process.env.NODE_ENV === 'production' ? './schema.graphql' : 'http://127.0.0.1:8000/graphql',
  documents: ['src/**/*.{ts,tsx}'],
  generates,
  ignoreNoDocuments: true,
};

export default config;
