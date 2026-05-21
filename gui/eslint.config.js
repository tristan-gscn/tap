import js from '@eslint/js';
import ts from 'typescript-eslint';
import svelte from 'eslint-plugin-svelte';
import globals from 'globals';

export default ts.config(
  js.configs.recommended,
  ...ts.configs.recommended,
  ...svelte.configs.recommended,
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
    rules: {
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          args: 'all',
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_',
          ignoreRestSiblings: true,
        },
      ],
    },
  },
  {
    files: ['**/*.svelte', '**/*.svelte.ts', '**/*.svelte.js'],
    languageOptions: {
      parserOptions: {
        projectService: true,
        extraFileExtensions: ['.svelte'],
        parser: ts.parser,
      },
    },
    rules: {
      // Threlte's useGltf/useLoader return AsyncWritable stores that are meant to
      // be awaited directly in `{#await store}` blocks. Adding the `$` prefix
      // dereferences the store to `undefined` while loading and breaks rendering.
      'svelte/require-store-reactive-access': 'off',
    },
  },
  {
    ignores: ['dist/', 'build/', 'node_modules/', '.svelte-kit/'],
  },
);
