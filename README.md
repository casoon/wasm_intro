# Rust + WebAssembly Einstieg 🚀

**Artikel 1** der Rust + WebAssembly Serie für Webentwickler

📖 **[Zum vollständigen Artikel](https://jseidel.io/insights/rust-webassembly-einstieg-webentwickler)**

## Was ist das?

Dieses Repository demonstriert den einfachsten Einstieg in Rust + WebAssembly:
- Eine Rust-Funktion im Browser ausführen
- Bidirektionale Kommunikation zwischen JavaScript und Rust
- Praktisches Setup für weitere Experimente

## Quick Start

### Voraussetzungen
```bash
# Rust installieren (falls noch nicht vorhanden)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# WebAssembly Target hinzufügen
rustup target add wasm32-unknown-unknown

# wasm-pack installieren
cargo install wasm-pack
```

### Projekt ausführen
```bash
# 1. Repository klonen
git clone https://github.com/casoon/wasm_intro
cd wasm_intro

# 2. WebAssembly kompilieren
wasm-pack build --target web

# 3. Lokalen Server starten
npx serve .

# 4. Browser öffnen: http://localhost:5000
```

## Was passiert hier?

1. **Rust-Code** (`src/lib.rs`) wird zu WebAssembly kompiliert
2. **wasm-bindgen** erstellt automatisch JavaScript-Bindings
3. **Browser** lädt das WASM-Modul und führt Rust-Funktionen aus
4. **Performance**: Nahezu native Geschwindigkeit im Browser

## Artikelserie

Dies ist **Teil 1** einer umfassenden Serie:

- [x] **Teil 1: Einstieg** (dieses Repo) - Grundlagen und erstes "Hello World"
- [ ] **Teil 2: Bildoptimierung** - `image`-Crate im Browser nutzen  
- [ ] **Teil 3: PDF-Generierung** - Dokumente direkt im Browser erstellen
- [ ] **Teil 4: Volltextsuche** - Alternative zu ElasticSearch mit WASM
- [ ] **Teil 5: E-Commerce Logic** - Checkout-Berechnungen in Rust
- [ ] **Teil 6: Browser-Datenbank** - SQLite mit WebAssembly

Jeder Teil enthält ein vollständiges Repository zum Ausprobieren.

## Entwicklung

### Nützliche Befehle
```bash
# Code formatieren
cargo fmt

# Linting
cargo clippy

# Tests ausführen (Rust-seitig)
cargo test

# WASM für verschiedene Targets kompilieren
wasm-pack build --target web      # Für Browser
wasm-pack build --target bundler  # Für Webpack etc.
wasm-pack build --target nodejs   # Für Node.js
```

### Projektstruktur
```
wasm_intro/
├── src/
│   └── lib.rs           # Rust-Code mit WASM-Exports
├── pkg/                 # Generierte WASM-Ausgabe (nach Build)
├── index.html          # Browser-Demo
├── Cargo.toml          # Rust-Konfiguration
└── README.md
```

## Nächste Schritte

- 🔧 Erweitere die `greet`-Funktion um mehr Parameter
- 📊 Experimentiere mit komplexeren Datentypen (Arrays, Structs)
- ⚡ Miss die Performance vs. reinem JavaScript
- 🎯 Schaue dir Teil 2 der Serie an (Link folgt)

## Links & Ressourcen

- 📄 [Vollständiger Artikel](https://jseidel.io/insights/rust-webassembly-einstieg-webentwickler)
- 📚 [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)
- 🛠️ [wasm-pack Dokumentation](https://rustwasm.github.io/wasm-pack/)
- 🌐 [MDN WebAssembly Guide](https://developer.mozilla.org/en-US/docs/WebAssembly)

## Autor

**Jörn Seidel** - Freelance Developer & Blogger
- Website: [jseidel.io](https://jseidel.io)
- GitHub: [@casoon](https://github.com/casoon)

