# 🦀 Instalación y Configuración del Entorno de Desarrollo

Guía completa para instalar rustup, toolchains oficiales (cargo, rustc, clippy, rustfmt) y configurar Visual Studio Code con rust-analyzer y CodeLLDB.

---

## 🛠️ Requisitos e Instalación

### 1. Instalar Rust vía `rustup` (Oficial)
* **Linux / macOS:**
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
* **Windows:**
  Descarga el instalador oficial desde [rustup.rs](https://rustup.rs/) (requiere los Build Tools de C++ de Visual Studio o toolchain GNU).

### 2. Verificar la Instalación en Terminal
```bash
rustc --version
cargo --version
```

### 3. Herramientas de Calidad de Código
```bash
rustup component add clippy rustfmt
```

### 4. Extensiones Recomendadas para Visual Studio Code
1. **rust-analyzer** (Soporte oficial del lenguaje, autocompletado y tipos inferidos).
2. **CodeLLDB** (Depurador nativo paso a paso).
3. **Even Better TOML** (Resaltado y validación de `Cargo.toml`).

---

📺 **Mira la clase completa en YouTube:** [Canal @gersonjadev](https://youtube.com/@gersonjadev)  
👨‍🏫 **Instructor:** Gerson Castellanos ([@gersonjadev](https://github.com/gersonja))
