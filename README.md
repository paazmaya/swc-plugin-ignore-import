# SWC plugin to ignore and remove certain imports based on configuration

[![Rust CI](https://github.com/paazmaya/swc-plugin-ignore-import/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/paazmaya/swc-plugin-ignore-import/actions/workflows/build-and-test.yml)
[![Code Smells](https://sonarcloud.io/api/project_badges/measure?project=paazmaya_swc-plugin-ignore-import&metric=code_smells)](https://sonarcloud.io/summary/new_code?id=paazmaya_swc-plugin-ignore-import)
[![codecov](https://codecov.io/gh/paazmaya/swc-plugin-ignore-import/graph/badge.svg?token=T0lMtm9GSf)](https://codecov.io/gh/paazmaya/swc-plugin-ignore-import)

Inspired by [`babel-plugin-ignore-import`](https://www.npmjs.com/package/babel-plugin-ignore-import), and since I needed this functionality it was a blocker to moving fully utilizing [SWC](https://swc.rs/).

```js
import { transformSync } from "@swc/core";

const output = transformSync(input, {
  jsc: {
    experimental: {
      plugins: [
        [
          "swc-plugin-ignore-import",
          {
            pattern: ".scss$",
          }
        ]
      ],
    },
  },
});
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
