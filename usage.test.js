import path from "node:path";
import { describe, it } from "node:test";
import assert from "node:assert";

import { transformSync } from "@swc/core";

const pluginPath = path.resolve("target/wasm32-wasip1/debug/swc_plugin_ignore_import.wasm");

describe('SWC Plugin Ignore Import', () => {
  it('should remove specified imports', () => {
    const input = `
import "@exact/package-name";
import "keep-this";
`;

    const output = transformSync(input, {
      jsc: {
        parser: {
          syntax: "ecmascript",
        },
        experimental: {
          plugins: [
            [
              pluginPath,
              {
                pattern: "@exact/package-name",
              }
            ]
          ],
        },
      },
    });
    const expected = `
import "keep-this";
    `.trim();

    assert.strictEqual(output.code.trim(), expected);
  });

  it('should remove .scss imports', () => {
const input = `
import "styles.scss";
import "keep-this";
`;

    const output = transformSync(input, {
      jsc: {
        parser: {
          syntax: "ecmascript",
        },
        experimental: {
          plugins: [
            [
              pluginPath,
              {
                pattern: ".scss$",
              }
            ]
          ],
        },
      },
    });
    const expected = `
import "keep-this";
    `.trim();

    assert.strictEqual(output.code.trim(), expected);
  });

  it('should remove same word starting imports', () => {
const input = `
import "jquery"; // Still needed
import "react";
import "react-dom";
import "other-router";
import "react-router";
import "react-router-dom";
import "keep-this";
`;

    const output = transformSync(input, {
      jsc: {
        parser: {
          syntax: "ecmascript",
        },
        experimental: {
          plugins: [
            [
              pluginPath,
              {
                pattern: "^react",
              }
            ]
          ],
        },
      },
    });
    const expected = `
import "jquery"; // Still needed
import "other-router";
import "keep-this";
    `.trim();

    assert.strictEqual(output.code.trim(), expected);
  });
});