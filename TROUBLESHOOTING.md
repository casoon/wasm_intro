# Troubleshooting 🔧

Häufige Probleme und Lösungen bei der Entwicklung mit Rust + WebAssembly.

## Setup-Probleme

### "wasm32-unknown-unknown target not found"
```bash
rustup target add wasm32-unknown-unknown
```

### "wasm-pack not found"
```bash
cargo install wasm-pack
```

### "serve command not found"
```bash
npm install -g serve
# oder
npx serve .
```

## Build-Probleme

### Build schlägt fehl mit "linker error"
Stelle sicher, dass alle Abhängigkeiten korrekt installiert sind:
```bash
rustup update
wasm-pack --version
```

### "Package.json missing fields"
Das ist nur eine Warnung. Du kannst sie ignorieren oder fehlende Felder in `Cargo.toml` ergänzen:
```toml
[package]
name = "wasm_intro"
version = "0.1.0"
description = "Rust + WebAssembly Einstieg"
repository = "https://github.com/dein-username/wasm_intro"
license = "MIT"
```

## Browser-Probleme

### "Failed to fetch WASM module"
- Verwende einen lokalen Server (nicht `file://`)
- CORS-Probleme: immer `npx serve .` oder ähnliches verwenden

### Module lädt nicht
- Überprüfe, ob `pkg/` Ordner existiert und Dateien enthält
- Browser-Konsole öffnen (F12) für Fehlermeldungen
- Stelle sicher, dass der Build erfolgreich war

### JavaScript-Fehler bei Import
```javascript
// Korrekt:
import init, { greet } from "./pkg/wasm_intro.js";

// Falsch (häufiger Fehler):
import init, { greet } from "./pkg/wasm-intro.js"; // Unterstrich, nicht Bindestrich!
```

## Performance-Probleme

### WASM scheint nicht schneller als JavaScript
- Verwende `--release` Build für Produktionsversion:
  ```bash
  wasm-pack build --target web --release
  ```
- WebAssembly ist vor allem bei rechenintensiven Aufgaben schneller
- Kleine Funktionen haben JS-Interop-Overhead

### Fibonacci-Vergleich zeigt schlechte Performance
Das ist normal! Der naive rekursive Fibonacci-Algorithmus ist exponentiell langsam. Für echte Performance-Tests verwende iterative Algorithmen oder komplexere Berechnungen.

## Development-Workflow

### Änderungen werden nicht übernommen
Nach Änderungen am Rust-Code:
1. `wasm-pack build --target web`
2. Browser-Cache leeren (Cmd/Ctrl + Shift + R)
3. Seite neu laden

### Tests laufen nicht
```bash
# Für Rust-Tests:
cargo test

# Für WebAssembly-Tests (benötigt wasm-bindgen-test):
cargo install wasm-pack
wasm-pack test --node
```

## GitHub Actions Probleme

### Build schlägt in CI fehl
- Überprüfe, dass alle Abhängigkeiten in der `build-and-deploy.yml` korrekt sind
- Stelle sicher, dass `cargo fmt --check` und `cargo clippy` lokal bestehen

### GitHub Pages funktioniert nicht
1. Repository-Einstellungen → Pages
2. Source: "GitHub Actions" auswählen
3. Workflow muss mindestens einmal erfolgreich gelaufen sein

## Erweiterte Debugging-Tipps

### Console-Logging aus Rust
```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn debug_function() {
    log("Debug-Nachricht aus Rust!");
}
```

### Browser-Devtools verwenden
- F12 → Console: JavaScript-Fehler und Logs
- F12 → Network: WASM-Ladeprobleme
- F12 → Sources: JavaScript-Debugging

### WASM-Module inspizieren
Moderne Browser haben WebAssembly-Support in den Devtools:
- Chrome: Sources → WebAssembly
- Firefox: Debugger → wasm://

## Hilfreiche Links

- [wasm-pack Troubleshooting](https://rustwasm.github.io/wasm-pack/book/debugging.html)
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
- [WebAssembly MDN](https://developer.mozilla.org/en-US/docs/WebAssembly)

---

Wenn du weitere Probleme hast, öffne ein Issue im Repository! 🚀