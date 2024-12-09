import assert from "node:assert";
import test from "node:test";
import path from "node:path";

import { transformSync } from "@swc/core";

// Example code to be transformed
const code = `
import "some-pattern";
import "keep-this";
console.log("Hello, world!");
`;

// Transform the code using SWC with the Wasm plugin
const output = transformSync(code, {
  jsc: {
    parser: {
      syntax: "ecmascript",
    },
    experimental: {
      plugins: [
        [
          path.resolve("target/wasm32-wasip1/release/swc_plugin_ignore_import.wasm"),
          {
            pattern: "some-pattern",
          }
        ]
      ],
    },
  },
});

console.log(output.code);

// Node.js test runner test case
test('SWC plugin should remove specified imports', () => {
  const expectedOutput = `
import "keep-this";
console.log("Hello, world!");
  `.trim();

  assert.strictEqual(output.code.trim(), expectedOutput);
});