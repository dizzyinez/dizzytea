import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
        //schema: 'https://localhost:4000/graphql',
        schema: 'schema.graphql',
        documents: ['src/**/*.tsx'],
        ignoreNoDocuments: true,
        generates: {
                './src/gql/': {
                        preset: 'client',
                        plugins: [],
                },
                './src/introspection.json': {
                        plugins: ['urql-introspection'],
                        config: {
                                minify: true
                        },
                },
        },
};
export default config;
