# 🦀 Rust + WebAssembly Serie für Webentwickler

**Vollständige Artikelserie** mit praktischen Beispielen und Live-Demos

📖 **[Zur Artikelserie auf jseidel.io](https://jseidel.io/insights/rust-webassembly-einstieg-webentwickler)**

## 📚 Serie-Überblick

Diese Repository-Serie zeigt praxisnahe Rust + WebAssembly Projekte für Webentwickler. Jeder Teil ist ein vollständiges, lauffähiges Projekt mit eigenständiger Dokumentation.

### 🎯 Was du lernst:
- Rust-Code im Browser mit nahezu nativer Performance ausführen  
- Komplexe Aufgaben clientseitig lösen (statt Server-Abhängigkeiten)
- Performance-kritische Algorithmen in WASM implementieren
- Praktische Integrationen in moderne Web-Apps

---

## 🗂️ Alle Teile der Serie

| Teil | Thema | Status | Live-Demo | Artikel |
|------|-------|--------|-----------|----------|
| **01** | [**Grundlagen & Einstieg**](./part-01-grundlagen/) | ✅ Fertig | [🔗 Demo](https://casoon.github.io/wasm_intro/part-01-grundlagen/) | [📄 Artikel](https://jseidel.io/insights/rust-webassembly-einstieg-webentwickler) |
| **02** | [**Bildoptimierung**](./part-02-bildoptimierung/) | 🚧 Geplant | 🔗 Demo | 📄 Artikel |
| **03** | [**PDF-Generierung**](./part-03-pdf-generation/) | 🚧 Geplant | 🔗 Demo | 📄 Artikel |
| **04** | [**Volltextsuche**](./part-04-volltextsuche/) | 🚧 Geplant | 🔗 Demo | 📄 Artikel |
| **05** | [**E-Commerce Logic**](./part-05-ecommerce/) | 🚧 Geplant | 🔗 Demo | 📄 Artikel |
| **06** | [**Browser-Datenbank**](./part-06-browser-database/) | 🚧 Geplant | 🔗 Demo | 📄 Artikel |

---

## 🚀 Quick Start für einen Teil

```bash
# 1. Repository klonen
git clone https://github.com/casoon/wasm_intro
cd wasm_intro

# 2. Einen Teil auswählen (z.B. Teil 1)
cd part-01-grundlagen

# 3. Abhängigkeiten installieren (einmalig)
make install

# 4. Entwicklung starten (Build + Server)
make dev

# 5. Browser öffnen: http://localhost:5000
```

---

## 🛠️ Globale Befehle

Jeder Teil unterstützt die gleichen Befehle:

```bash
make help           # Alle verfügbaren Befehle anzeigen
make install        # Abhängigkeiten installieren  
make dev            # Build + lokaler Server (Hauptbefehl)
make build          # WASM kompilieren
make test           # Tests ausführen
make clean          # Aufräumen
make check          # Code-Qualität prüfen (Format, Lint, Test)
```

---

## 📖 Serie-Details

### **Teil 1: Grundlagen & Einstieg** ✅
**Ordner:** [`part-01-grundlagen/`](./part-01-grundlagen/)  
**Was du lernst:** Erste Rust-Funktionen im Browser ausführen, Performance-Vergleiche, verschiedene Datentypen

**Features:**
- 🎯 Einfache Grußfunktion (wie im Artikel)
- ⚡ Performance-Vergleich: Fibonacci (Rust vs. JavaScript)
- 🔢 Array-Verarbeitung und String-Manipulation
- 🧮 Mathematische Funktionen (Primzahl-Test)
- 🐛 Console-Logging aus Rust heraus
- ✅ Vollständige Test-Suite

### **Teil 2: Bildoptimierung** 🚧
**Ordner:** [`part-02-bildoptimierung/`](./part-02-bildoptimierung/)  
**Was du lernst:** `image`-Crate im Browser nutzen, Bildformate konvertieren, Filter anwenden

### **Teil 3: PDF-Generierung** 🚧  
**Ordner:** [`part-03-pdf-generation/`](./part-03-pdf-generation/)  
**Was du lernst:** PDFs direkt im Browser erstellen, ohne Server-Dependencies

### **Teil 4: Volltextsuche** 🚧
**Ordner:** [`part-04-volltextsuche/`](./part-04-volltextsuche/)  
**Was du lernst:** Suchindex im Browser, Alternative zu ElasticSearch für kleine bis mittlere Datenmengen

### **Teil 5: E-Commerce Logic** 🚧
**Ordner:** [`part-05-ecommerce/`](./part-05-ecommerce/)  
**Was du lernst:** Checkout-Berechnungen, Steuer-Logic, Währungsumrechnung in Rust

### **Teil 6: Browser-Datenbank** 🚧
**Ordner:** [`part-06-browser-database/`](./part-06-browser-database/)  
**Was du lernst:** SQLite mit WASM, lokale Datenbank im Browser

---

## 🏗️ Repository-Struktur

```
rust-wasm-serie/
├── README.md                    # Diese Datei (Serie-Übersicht)
├── .github/workflows/           # CI/CD für alle Teile
├── docs/                        # Gemeinsame Dokumentation
├── TROUBLESHOOTING.md           # Häufige Probleme & Lösungen
│
├── part-01-grundlagen/          # ✅ Fertig
│   ├── src/lib.rs
│   ├── index.html
│   ├── Cargo.toml
│   ├── Makefile
│   └── WARP.md
│
├── part-02-bildoptimierung/     # 🚧 Geplant
├── part-03-pdf-generation/      # 🚧 Geplant  
├── part-04-volltextsuche/       # 🚧 Geplant
├── part-05-ecommerce/           # 🚧 Geplant
└── part-06-browser-database/    # 🚧 Geplant
```

---

## 🎯 Für wen ist diese Serie?

### **Freelancer & Webentwickler**
- Erweitere dein Skillset um High-Performance Browser-Anwendungen
- Biete Services an, die bisher nur mit großen Backend-Infrastrukturen möglich waren
- Reduziere Server-Kosten durch clientseitige Verarbeitung

### **JavaScript-Entwickler**  
- Lerne Rust in praktischen, webfokussierten Projekten
- Performance-kritische Teile deiner Apps optimieren
- Neue Türen für komplexe Browser-Anwendungen öffnen

### **Rust-Entwickler**
- Bringe deine Rust-Skills ins Web
- Praktische WebAssembly-Integration lernen  
- Echte Projekte statt nur "Hello World"

---

## 🔗 Links & Ressourcen

### **Serie-spezifisch:**
- 🌐 **Live-Demos:** [casoon.github.io/wasm_intro](https://casoon.github.io/wasm_intro)
- 📄 **Blog-Artikel:** [jseidel.io/insights](https://jseidel.io/insights)
- 💬 **Diskussionen:** [GitHub Discussions](https://github.com/casoon/wasm_intro/discussions)

### **Allgemeine Ressourcen:**
- 📚 [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)
- 🛠️ [wasm-pack Dokumentation](https://rustwasm.github.io/wasm-pack/)
- 🌐 [MDN WebAssembly Guide](https://developer.mozilla.org/en-US/docs/WebAssembly)
- 🦀 [Rust Learn](https://www.rust-lang.org/learn)

---

## 👨‍💻 Autor

**Jörn Seidel** - Freelance Developer & Blogger  
Spezialisiert auf moderne Webtechnologien und Performance-Optimierung

- 🌐 **Website:** [jseidel.io](https://jseidel.io)
- 🐙 **GitHub:** [@casoon](https://github.com/casoon)  
- 📧 **Kontakt:** Über die Website

---

*Diese Serie wird regelmäßig erweitert. Star das Repository, um Updates nicht zu verpassen! ⭐*

