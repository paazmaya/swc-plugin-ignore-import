import {
  transformSync
} from '@swc/core';

import PluginIgnoreImport from '../index.ts';

describe('PluginIgnoreImport', () => {
  test('strips out scss when used as a plugin', () => {
    const src = `
   import React from 'react';
   import './styles.scss';
   `;

    const {
      code
    } = transformSync(src, {
      plugin: (m) =>
        new PluginIgnoreImport({
          pattern: /\.scss$/
        }).visitProgram(m)
    });
    expect(code).toBe(`import React from 'react';
;
`);
  });

});
