# Teil 1: Rust + WebAssembly Grundlagen 🚀

**Artikel 1** der Rust + WebAssembly Serie für Webentwickler

📖 **[Zum vollständigen Artikel](https://jseidel.io/insights/rust-webassembly-einstieg-webentwickler)**  
🔙 **[Zurück zur Serie-Übersicht](../README.md)**

## Was lernst du hier?

Dieses Projekt demonstriert den einfachsten Einstieg in Rust + WebAssembly:
- 🎯 Eine Rust-Funktion im Browser ausführen
- ⚡ Performance-Vergleiche zwischen Rust und JavaScript
- 🔢 Bidirektionale Kommunikation zwischen JavaScript und Rust
- 🧮 Verschiedene Datentypen und Funktionen in WASM
- 🐛 Debugging und Console-Logging aus Rust heraus

## 🎮 Live-Demo

👉 **[Direkt ausprobieren](https://casoon.github.io/wasm_intro/part-01-grundlagen/)**

## Quick Start

### Voraussetzungen (einmalig)
```bash
# Rust installieren (falls noch nicht vorhanden)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# WebAssembly Target hinzufügen
rustup target add wasm32-unknown-unknown

# wasm-pack installieren
cargo install wasm-pack
```

### Projekt starten
```bash
# 1. Repository klonen und in Teil 1 navigieren
git clone https://github.com/casoon/wasm_intro
cd wasm_intro/part-01-grundlagen

# 2. Entwicklung starten (kompiliert WASM + startet Server)
make dev

# 3. Browser öffnen: http://localhost:5000
```

## 🛠️ Verfügbare Befehle

```bash
make help           # Alle verfügbaren Befehle anzeigen
make install        # Abhängigkeiten installieren  
make dev            # Build + lokaler Server (Hauptbefehl)
make build          # WASM kompilieren
make test           # Tests ausführen
make clean          # Aufräumen
make check          # Code-Qualität prüfen (Format, Lint, Test)
```

## 🧪 Was passiert in der Demo?

Die interaktive Demo zeigt **6 verschiedene WASM-Funktionen**:

### 1. **Einfache Grußfunktion**
- Rust-String-Verarbeitung im Browser
- Wie im Artikel beschrieben

### 2. **Performance-Vergleich: Fibonacci** ⚡
- **Rust vs. JavaScript** Performance-Messung
- Zeigt WASM-Geschwindigkeitsvorteile konkret
- Probiere Werte wie 40-45 für spürbare Unterschiede

### 3. **Array-Summe**
- Datenübertragung zwischen JS und Rust
- Arbeiten mit `Float64Array`

### 4. **String umkehren**
- Unicode-sichere String-Manipulation in Rust
- UTF-8 Unterstützung

### 5. **Primzahl-Test**
- Mathematische Algorithmen in WASM
- Boolean-Rückgabe nach JavaScript

### 6. **Debug-Nachrichten**
- Console-Logging aus Rust heraus
- Wichtig für Debugging komplexerer Apps

## 🏗️ Projekt-Architektur

```
part-01-grundlagen/
├── src/
│   └── lib.rs           # Rust-Code mit WASM-Exports
├── pkg/                 # Generierte WASM-Ausgabe (nach Build)
├── index.html          # Interaktive Browser-Demo
├── Cargo.toml          # Rust-Konfiguration
├── Makefile            # Entwicklungshelfer
└── WARP.md            # Warp-Terminal Integration
```

### Wichtige Dateien:

**`src/lib.rs`** - Alle WASM-Funktionen:
```rust
#[wasm_bindgen]
pub fn greet(name: &str) -> String { ... }

#[wasm_bindgen] 
pub fn fibonacci(n: u32) -> u64 { ... }

// + 4 weitere Funktionen
```

**`index.html`** - Browser-Demo mit:
- Modernes, responsives Design
- Performance-Messungen in Echtzeit
- Interaktive Bereiche für alle Funktionen

**`Cargo.toml`** - WASM-Konfiguration:
```toml
[lib]
crate-type = ["cdylib"]  # Wichtig für WASM!

[dependencies]
wasm-bindgen = "0.2"
```

## 🔬 Was passiert technisch?

1. **Rust-Compilation**: `cargo` kompiliert Rust nach WASM-Bytecode
2. **wasm-bindgen**: Erstellt automatisch JavaScript-Bindings
3. **Browser-Integration**: ES6-Module laden das WASM-Modul
4. **Execution**: JavaScript ruft Rust-Funktionen direkt auf

```javascript
import init, { greet, fibonacci } from "./pkg/wasm_intro.js";

await init();                           // WASM-Modul initialisieren
const result = greet("WebAssembly");    // Rust-Funktion aufrufen
```

## ⚡ Performance-Erkenntnisse

Die Demo zeigt reale Performance-Unterschiede:
- **Kleine Funktionen**: JavaScript oft vergleichbar (JIT-Compiler)
- **Rechenintensive Algorithmen**: WASM deutlich schneller
- **Fibonacci(40+)**: Oft 2-5x Geschwindigkeitsgewinn mit Rust

> **Tipp**: In Produktions-Apps verwende `wasm-pack build --release` für maximale Performance!

## 🧪 Tests

Das Projekt enthält eine vollständige Test-Suite:

```bash
make test

# Output:
# running 5 tests
# test tests::test_greet ... ok
# test tests::test_fibonacci ... ok  
# test tests::test_sum_array ... ok
# test tests::test_reverse_string ... ok
# test tests::test_is_prime ... ok
```

Tests decken alle WASM-Funktionen ab und stellen sicher, dass Änderungen keine Regressionen verursachen.

## 🔧 Troubleshooting

Häufige Probleme und Lösungen findest du in der [TROUBLESHOOTING.md](../TROUBLESHOOTING.md).

**Die häufigsten Probleme:**
- ❌ `serve command not found` → `npm install -g serve`
- ❌ WASM lädt nicht → Lokaler Server verwenden (nicht `file://`)
- ❌ Änderungen werden nicht übernommen → `make build` + Browser-Cache leeren

## 🎯 Nächste Schritte

### Experimentiere mit dem Code:
- 🔧 Erweitere die `greet`-Funktion um weitere Parameter
- 📊 Implementiere andere Algorithmen (z.B. Quicksort)
- ⚡ Miss Performance bei verschiedenen Datengrößen
- 🌐 Integriere das WASM-Modul in deine eigene Web-App

### Weitermachen mit der Serie:
- 📖 **[Teil 2: Bildoptimierung](../part-02-bildoptimierung/)** (geplant)
- 🔙 **[Zurück zur Serie-Übersicht](../README.md)**

## 📚 Ressourcen

- 📄 [Vollständiger Artikel zu diesem Teil](https://jseidel.io/insights/rust-webassembly-einstieg-webentwickler)
- 📚 [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)
- 🛠️ [wasm-pack Dokumentation](https://rustwasm.github.io/wasm-pack/)
- 🌐 [MDN WebAssembly Guide](https://developer.mozilla.org/en-US/docs/WebAssembly)

---

**Viel Erfolg beim Experimentieren mit Rust + WebAssembly! 🦀✨**