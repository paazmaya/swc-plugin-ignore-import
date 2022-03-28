# SWC plugin to ignore and remove certain imports based on configuration

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
