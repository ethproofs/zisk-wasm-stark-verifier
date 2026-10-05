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

Node resolves the package to a separate build that loads the `.wasm` file from disk,
so it works with both `import` and `require`:

```javascript
import { readFileSync } from 'node:fs';
import { verify_stark } from '@ethproofs/zisk-wasm-stark-verifier';
// or: const { verify_stark } = require('@ethproofs/zisk-wasm-stark-verifier');

const isValid = verify_stark(
  readFileSync('zisk.proof.bin'),
  readFileSync('vadcop_final.verkey.bin')
);
```

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

# Build only the targets that get published (pkg/ and pkg-node/)
npm run build:publish
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
npm run build:publish
npm publish
```

`prepublishOnly` stops the publish if `pkg/` or `pkg-node/` hasn't been built.
