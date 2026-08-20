# ZisK Wasm Stark Verifier

WebAssembly bindings for the ZisK STARK verifier.

## Overview

This module builds the `verify_stark` function from `zisk-verifier` (ZisK `v1.1.0-alpha`) into WebAssembly, enabling STARK proof verification to run directly in both web browsers and Node.js environments.

## Usage

> **Breaking change in 0.2.0.** `verify_stark` now verifies against the `vkBytes`
> you pass in. Earlier versions ignored that argument and used the verification key
> embedded in the proof, which only proves the proof is internally consistent — not
> that it came from the expected program. Callers must now pass the 32-byte key from
> the trusted setup (`vadcop_final_compressed.verkey.bin` for a minimal proof,
> otherwise `vadcop_final.verkey.bin`). A key of any other length throws, and a key
> that doesn't match returns `false`.

### Installation

```bash
npm install @ethproofs/zisk-wasm-stark-verifier
```

### React Integration

```typescript
import init, { main, verify_stark } from '@ethproofs/zisk-wasm-stark-verifier';

await init(); // Initialize WASM (if needed)
main(); // Initialize panic hook

// Verify a proof
const isValid = verify_stark(proofBytes, vkBytes);
```

### Node.js Usage

```javascript
const { main, verify_stark } = require('@ethproofs/zisk-wasm-stark-verifier');

// The Node.js version initializes automatically

main(); // Initialize panic hook
const result = verify_stark(proofBytes, vkBytes);
```

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
# Build for all targets
npm run build:all
```

### Node.js Example

```bash
npm run test:node
```

This runs the Node.js example that loads proof and verification key files from the filesystem and verifies them.

### Browser Example

```bash
npm run test
```

This starts a local HTTP server at `http://localhost:8080` with a browser example that demonstrates:

- Loading the WASM module in a browser environment
- File upload interface for proof and verification key files
- Interactive STARK proof verification
- Performance metrics and detailed logging
- Error handling and user feedback

The browser example provides a complete UI for testing the WASM verifier with drag-and-drop file selection and real-time verification results.

**Note:** The browser example requires files to be served over HTTP due to WASM CORS restrictions. The included server script handles this automatically.
