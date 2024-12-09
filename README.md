# SWC plugin to ignore and remove certain imports based on configuration

[![Node.js v22 CI](https://github.com/paazmaya/swc-plugin-ignore-import/actions/workflows/linting-and-unit-testing.yml/badge.svg)](https://github.com/paazmaya/swc-plugin-ignore-import/actions/workflows/linting-and-unit-testing.yml)
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


```js
const swc = require("@swc/core");
const { loadWasmPlugin } = require("@swc/plugin-loader");
const fs = require("fs");
const path = require("path");

// Path to the compiled WASM plugin
const pluginPath = path.resolve(
  __dirname,
  "swc_plugin_ignore_import.wasm"
);

const wasmPlugin = await loadWasmPlugin(fs.readFileSync(pluginPath));

const { code } = swc.transformSync(src, {
  filename: "source-file-name-for-sourcemap.js",
  sourceMaps: true,
  jsc: {
    parser: {
      syntax: "ecmascript",
    },
    target: "es2015",
  },
  plugin: (m) => {
    // Apply the WASM plugin to the SWC transform
    return wasmPlugin(m, {
      pattern: "\\.s?css$", // Plugin-specific options
    });
  },
});

console.log(code);

```

## Code style

```sh
cargo fmt
```

## Version history

[Changes happening across different versions and upcoming changes are tracked in the `CHANGELOG.md` file.](CHANGELOG.md)

## License

Licensed under [the MIT license](LICENSE).

Copyright (c) [Juga Paazmaya](https://paazmaya.fi) <paazmaya@yahoo.com>
