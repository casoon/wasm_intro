# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is a Rust + WebAssembly introductory project that demonstrates how to:
- Compile Rust code to WebAssembly 
- Bind Rust functions for JavaScript consumption using `wasm-bindgen`
- Serve and run WebAssembly modules in the browser

The project structure is minimal:
- `src/lib.rs` - Contains Rust functions exported to WebAssembly
- `index.html` - Browser demo that imports and uses the WebAssembly module
- `Cargo.toml` - Rust project configuration with `wasm-bindgen` dependency
- `pkg/` - Generated WebAssembly output directory (created by wasm-pack build)

## Development Commands

### Prerequisites Setup
```bash
# Add WebAssembly target to Rust toolchain
rustup target add wasm32-unknown-unknown

# Install wasm-pack (if not already installed)
cargo install wasm-pack
```

### Build Commands
```bash
# Build the WebAssembly module for web target
wasm-pack build --target web

# Build for different targets
wasm-pack build --target bundler    # For webpack/bundler integration
wasm-pack build --target nodejs     # For Node.js usage
```

### Development Server
```bash
# Serve the project locally (requires build first)
npx serve .

# Alternative local servers
python3 -m http.server 8000
# or
php -S localhost:8000
```

After running a server, open `http://localhost:5000` (or respective port) in browser.

### Code Quality
```bash
# Format Rust code
cargo fmt

# Run linter
cargo clippy

# Basic Rust unit tests (currently none defined)
cargo test
```

### Testing WebAssembly
To add WebAssembly-specific tests, first add to `Cargo.toml`:
```toml
[dev-dependencies]
wasm-bindgen-test = "0.2"
```

Then run:
```bash
# Test in Node.js environment
wasm-pack test --node

# Test in browser (requires chromedriver)
wasm-pack test --chrome --headless
```

## Architecture

### WebAssembly Interface
The project uses `wasm-bindgen` to create JavaScript bindings for Rust functions. Functions decorated with `#[wasm_bindgen]` in `src/lib.rs` become available in JavaScript.

### Build Process
1. `wasm-pack build` compiles Rust to WebAssembly
2. Generates TypeScript definitions and JavaScript bindings in `pkg/`
3. Produces optimized `.wasm` binary and accompanying JS/TS files
4. The `pkg/` directory contains everything needed to import the module in web applications

### Browser Integration
The HTML file uses ES6 modules to import the generated WebAssembly:
- `import init, { greet } from "./pkg/wasm_intro.js"` 
- `await init()` initializes the WebAssembly module
- Exported Rust functions become available as JavaScript functions

## Important Notes

- The project requires a local server due to CORS restrictions with WebAssembly modules
- Build artifacts in `pkg/` are generated and should not be committed to version control
- The crate type is set to `cdylib` for dynamic library output suitable for WebAssembly
- Target architecture is `wasm32-unknown-unknown` for web deployment