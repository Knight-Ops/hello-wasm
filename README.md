# Hello, Wasm (Extism Plugin)

This is a simple "Hello, Wasm" plugin built using [Extism](https://extism.org).

## Prerequisites

- [Rust](https://rustup.rs/)
- `wasm32-unknown-unknown` target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

## Building

To build the plugin:

```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled Wasm module will be located at:
`target/wasm32-unknown-unknown/release/hello_wasm.wasm`

## Testing (with Extism CLI)

If you have the [Extism CLI](https://github.com/extism/cli) installed, you can test the plugin like this:

```bash
extism call target/wasm32-unknown-unknown/release/hello_wasm.wasm greet --input "Antigravity"
```

Expected output:
`Hello, Antigravity from Wasm!`

## Continuous Integration & Deployment

This repository includes a GitHub Action (`.github/workflows/publish.yml`) that automatically:
1. Compiles the Wasm module on every push to `main` or when a new tag is created.
2. Publishes the compiled `.wasm` file to **GitHub Container Registry (GHCR)** as an OCI artifact.

### Pulling from GHCR

You can pull the compiled Wasm module using [ORAS](https://oras.land/):

```bash
oras pull ghcr.io/<your-username>/hello-wasm:main
```
