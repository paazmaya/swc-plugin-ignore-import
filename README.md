# SWC plugin to ignore and remove certain imports based on configuration

[![Node.js v20 CI](https://github.com/paazmaya/swc-plugin-ignore-import/actions/workflows/linting-and-unit-testing.yml/badge.svg)](https://github.com/paazmaya/swc-plugin-ignore-import/actions/workflows/linting-and-unit-testing.yml)
[![Code Smells](https://sonarcloud.io/api/project_badges/measure?project=paazmaya_swc-plugin-ignore-import&metric=code_smells)](https://sonarcloud.io/summary/new_code?id=paazmaya_swc-plugin-ignore-import)

Inspired from https://www.npmjs.com/package/babel-plugin-ignore-import and since I needed this functionality, as it was blocker was moving to SWC.

```js
const swc = require("@swc/core");
const PluginIgnoreImport = require("swc-plugin-ignore-import").default;

const { code } = swc.transformSync(src, {
  filename: "source-file-name-for-sourcemap.js",
  sourceMaps: true,
  jsc: {
    parser: {
      syntax: "ecmascript",
    },
    target: "es2015",
  },
  plugin: (m) =>
    new PluginIgnoreImport({
      pattern: /\.s?css$/,
    }).visitProgram(m),
});
```

## Code style

```sh
npx prettier --write index.ts
```

## Version history

[Changes happening across different versions and upcoming changes are tracked in the `CHANGELOG.md` file.](CHANGELOG.md)

## License

Licensed under [the MIT license](LICENSE).

Copyright (c) [Juga Paazmaya](https://paazmaya.fi) <paazmaya@yahoo.com>
