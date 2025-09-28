# Rust + WebAssembly Intro 🚀

Dies ist das Beispielprojekt für **Artikel 1** der Blogserie.

## Voraussetzungen
- Rust (`rustup`)
- wasm-pack
- Node.js / npm

## Setup
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

## Build
```bash
wasm-pack build --target web
```

## Starten
```bash
npx serve .
```

Danach im Browser öffnen: `http://localhost:5000`

