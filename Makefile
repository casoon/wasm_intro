.PHONY: help build serve clean test format lint check install dev

# Standard-Target
help: ## Zeigt verfügbare Befehle
	@echo "🦀 Rust + WebAssembly Entwicklungshelfer"
	@echo ""
	@echo "Verfügbare Befehle:"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-12s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install: ## Installiert alle Abhängigkeiten
	@echo "🔧 Installiere Abhängigkeiten..."
	@rustup target add wasm32-unknown-unknown
	@cargo install wasm-pack
	@echo "✅ Setup komplett!"

build: ## Kompiliert das WASM-Modul
	@echo "🔨 Baue WebAssembly-Modul..."
	@wasm-pack build --target web
	@echo "✅ Build erfolgreich!"

dev: build serve ## Build + Server starten (Hauptbefehl für Entwicklung)

serve: ## Startet lokalen Development-Server
	@echo "🚀 Starte Development-Server..."
	@echo "👉 Öffne http://localhost:5000 im Browser"
	@npx serve .

clean: ## Räumt Build-Artefakte auf
	@echo "🧹 Räume auf..."
	@rm -rf pkg/
	@rm -rf target/
	@echo "✅ Aufgeräumt!"

format: ## Formatiert Rust-Code
	@echo "💅 Formatiere Code..."
	@cargo fmt
	@echo "✅ Code formatiert!"

lint: ## Führt Clippy-Linting aus
	@echo "🔍 Führe Linting aus..."
	@cargo clippy -- -D warnings
	@echo "✅ Linting bestanden!"

test: ## Führt Tests aus
	@echo "🧪 Führe Tests aus..."
	@cargo test
	@echo "✅ Tests bestanden!"

check: format lint test ## Vollständige Code-Qualitätsprüfung
	@echo "✅ Alle Checks bestanden!"

# Targets für verschiedene Build-Varianten
build-bundler: ## Build für Bundler (Webpack etc.)
	@wasm-pack build --target bundler

build-nodejs: ## Build für Node.js
	@wasm-pack build --target nodejs

build-all: build build-bundler build-nodejs ## Alle Build-Varianten