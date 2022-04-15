import {
  transformSync
} from '@swc/core';

import {
  visitModuleItems
} from '../index.ts';

describe('visitModuleItems', () => {
  test('visits module items', () => {

    // 'ImportDeclaration'
    const input = [];
    const output = visitModuleItems(input, /something/i);

    expect(output).toStrictEqual(input);
  });
});
