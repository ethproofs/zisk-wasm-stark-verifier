# ZisK Wasm Stark Verifier

WebAssembly bindings for the ZisK STARK verifier.

## Overview

This module builds the `verify_stark` function from `zisk-verifier` (ZisK `v1.3.1-alpha`) into WebAssembly, enabling STARK proof verification to run directly in both web browsers and Node.js environments.

## Usage

> **Breaking change in 0.4.0.** ZisK `v1.3` proofs end with a tag naming the hash
> function they were built with, and `verify_stark` reads it to choose the verifier.
> Proofs from earlier ZisK versions don't have the tag and throw
> `unrecognized hash tag …`. Use 0.3.x to verify `v1.2` proofs.

### Installation

```bash
npm install @ethproofs/zisk-wasm-stark-verifier
```

### Browser (bundlers)

The package resolves to a build that imports the `.wasm` file as an ES module, so it
needs a bundler with WebAssembly support: webpack 5 with
`experiments: { asyncWebAssembly: true }` (this includes Next.js), or Vite with
[`vite-plugin-wasm`](https://github.com/Menci/vite-plugin-wasm). The module is
ready as soon as the import resolves. There's no `init()` to call.

```typescript
import { verify_stark } from '@ethproofs/zisk-wasm-stark-verifier';

const isValid = verify_stark(proofBytes, vkBytes);
```

### Node.js

The npm package ships only the bundler build, which plain Node can't load: `require`
fails because the package is ESM, and `import` fails on the `.wasm` file. There are
two ways to use it in Node.

**Use the npm package with an experimental flag.** Node can import the `.wasm` file
directly when run with `--experimental-wasm-modules`. This only works with `import`,
not `require`.

```javascript
// node --experimental-wasm-modules verify.mjs
import { readFileSync } from 'node:fs';
import { verify_stark } from '@ethproofs/zisk-wasm-stark-verifier';

const isValid = verify_stark(
  readFileSync('zisk.proof.bin'),
  readFileSync('vadcop_final.verkey.bin')
);
```

**Build the Node target from source.** This doesn't need a flag, and it works with
both `import` and `require`. It needs the [prerequisites](#prerequisites) installed.

```bash
git clone https://github.com/ethproofs/zisk-wasm-stark-verifier
cd zisk-wasm-stark-verifier
npm install
npm run build:node   # outputs pkg-node/
```

```javascript
import { verify_stark } from './zisk-wasm-stark-verifier/pkg-node/zisk_wasm_stark_verifier.js';
// or: const { verify_stark } = require('./zisk-wasm-stark-verifier/pkg-node/zisk_wasm_stark_verifier.js');
```

Copy the whole `pkg-node/` directory if you vendor it: the `.js` file loads the
`.wasm` file next to it at startup.

### API

#### `verify_stark(proofBytes: Uint8Array, vkBytes: Uint8Array): boolean`

- `proofBytes`: the proof file the ZisK prover writes with `--proof.save`.
- `vkBytes`: the 32-byte verification key from the trusted setup:
  `vadcop_final_compressed.verkey.bin` for a minimal proof, otherwise
  `vadcop_final.verkey.bin`. The key the prover appends to the proof is ignored. A
  proof checked against its own key only shows that it is internally consistent, not
  that it came from the program you expect.

Returns `true` if the proof verifies against `vkBytes` and `false` if it doesn't.
Throws if the key isn't 32 bytes, the proof length isn't a multiple of 8, the proof
is too short, or its hash tag isn't recognized.

`main()` is also exported. It installs a panic hook that logs Rust panics to the
console. It runs automatically when the module loads, so you don't need to call it.

## Testing

### Installation

```bash
npm install
```

### Prerequisites

- [Rust](https://0xpolygonhermez.github.io/zisk/getting_started/quickstart.html)
- [wasm-pack](https://github.com/drager/wasm-pack)

### Building

```bash
# Build for all targets (pkg/, pkg-node/ and pkg-web/)
npm run build:all

# Build only the target that gets published (pkg/)
npm run build
```

### Node.js Example

```bash
npm run test:node
```

This builds `pkg-node/` and verifies `proofs/zisk.proof.bin` against `vks/zisk.vk.bin`.

### Browser Example

```bash
npm run test
```

This builds `pkg-web/` and starts a local HTTP server at `http://localhost:8080` with a browser example that demonstrates:

- Loading the WASM module in a browser environment
- File inputs for proof and verification key files
- Interactive STARK proof verification
- Performance metrics and detailed logging
- Error handling and user feedback

**Note:** The browser example requires files to be served over HTTP due to WASM CORS restrictions. The included server script handles this automatically.

## Publishing

```bash
npm run build
npm publish
```

Only `pkg/` is published. `prepublishOnly` stops the publish if it hasn't been built.
