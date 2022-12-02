import paazmaya from 'eslint-config-paazmaya';
import tsParser from '@typescript-eslint/parser';
import tsPlugin from '@typescript-eslint/eslint-plugin';

export default [
  paazmaya,
  {
    plugins: {
      tsPlugin
    },
    languageOptions: {
      parser: tsParser
    },
    rules: {
      'func-style': 'off'
    }
  }
];
